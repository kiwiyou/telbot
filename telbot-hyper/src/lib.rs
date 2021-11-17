use std::io::Cursor;

use hyper::{body::Buf, client::HttpConnector, Body, Client, Request, Response};
use hyper_multipart_rfc7578::client::multipart::{self, Form};
use hyper_tls::HttpsConnector;
pub use telbot_types as types;
use types::{ApiResponse, FileMethod, JsonMethod, TelegramError, TelegramMethod};

#[derive(Clone)]
pub struct Api {
    base_url: String,
    client: Client<HttpsConnector<HttpConnector>>,
}

#[derive(Debug)]
pub enum Error {
    Telegram(TelegramError),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    Mime(mime::FromStrError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Self::Hyper(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Serde(e)
    }
}

impl From<mime::FromStrError> for Error {
    fn from(e: mime::FromStrError) -> Self {
        Self::Mime(e)
    }
}

impl Api {
    pub fn new(token: impl AsRef<str>) -> Self {
        Self {
            base_url: format!("https://api.telegram.org/bot{}/", token.as_ref()),
            client: Client::builder().build(HttpsConnector::new()),
        }
    }

    pub async fn send_json<Method: JsonMethod>(&self, method: &Method) -> Result<Method::Response> {
        let body = serde_json::to_vec(method)?;

        let request = Request::builder()
            .method(&hyper::Method::POST)
            .uri(format!("{}{}", self.base_url, Method::name()))
            .header("Content-Type", "application/json")
            .body(Body::from(body))
            .unwrap();

        let response = self.client.request(request).await?;
        Self::parse_response::<Method>(response).await
    }

    pub async fn send_file<Method: FileMethod>(&self, method: &Method) -> Result<Method::Response> {
        let url = format!("{}{}", self.base_url, Method::name());
        let files = method.files();
        let serialized = serde_json::to_value(method).unwrap();

        let mut form = Form::default();
        for (key, value) in serialized.as_object().unwrap() {
            if let Some(file) = files.as_ref().and_then(|map| map.get(key.as_str())) {
                // Form::set_body_convert requires reader to be 'static.
                form.add_reader_file_with_mime(
                    key,
                    Cursor::new(file.data.clone()),
                    &file.name,
                    file.mime.parse()?,
                );
            } else if let Some(value) = value.as_str() {
                form.add_text(key, value);
            } else {
                form.add_text(key, value.to_string());
            }
        }

        let request = Request::builder().method(&hyper::Method::POST).uri(url);
        let request = form
            .set_body_convert::<hyper::Body, multipart::Body>(request)
            .unwrap();
        let response = self.client.request(request).await?;
        Self::parse_response::<Method>(response).await
    }

    async fn parse_response<Method: TelegramMethod>(
        response: Response<Body>,
    ) -> Result<Method::Response> {
        let body = hyper::body::aggregate(response).await?;
        let tg_response: ApiResponse<_> = serde_json::from_reader(body.reader())?;
        match tg_response {
            ApiResponse::Ok { result } => Ok(result),
            ApiResponse::Err(e) => Err(Error::Telegram(e)),
        }
    }
}
