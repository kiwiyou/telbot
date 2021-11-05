use std::io::Read;

use multipart::client::lazy::Multipart;
pub use telbot_types as types;
use telbot_types::{ApiResponse, FileMethod, JsonMethod, TelegramError, TelegramMethod};
use worker::wasm_bindgen::JsValue;
use worker::{Fetch, Headers, Request, RequestInit, Response};

#[derive(Clone)]
pub struct Api {
    base_url: String,
}

impl Api {
    pub fn new(token: impl AsRef<str>) -> Self {
        Self {
            base_url: format!("https://api.telegram.org/bot{}/", token.as_ref()),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    TelegramError(TelegramError),
    Worker(worker::Error),
    Io(std::io::Error),
}

impl From<worker::Error> for Error {
    fn from(error: worker::Error) -> Self {
        Self::Worker(error.into())
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl Api {
    /// Send a JSON-serializable API request
    pub async fn send_json<Method: JsonMethod>(&self, method: &Method) -> Result<Method::Response> {
        let mut headers = Headers::new();
        headers.set("Content-Type", "application/json")?;
        let mut request = RequestInit::new();
        let payload = serde_json::to_string(&method).map_err(Into::<worker::Error>::into)?;
        request
            .with_method(worker::Method::Post)
            .with_body(Some(JsValue::from_str(&payload)))
            .with_headers(headers);

        let response = Fetch::Request(Request::new_with_init(
            &format!("{}{}", self.base_url, Method::name()),
            &request,
        )?)
        .send()
        .await?;

        Self::parse_response::<Method>(response).await
    }

    /// Send a JSON-serializable API request with files.
    pub async fn send_file<Method: FileMethod>(&self, method: &Method) -> Result<Method::Response> {
        let mut request = RequestInit::new();
        let value = serde_json::to_value(method).map_err(Into::<worker::Error>::into)?;
        let files = method.files();
        let mut multipart = Multipart::new();
        for (key, value) in value.as_object().unwrap() {
            if let Some(file) = files.as_ref().and_then(|map| map.get(key.as_str())) {
                multipart.add_stream(
                    key,
                    &file.data[..],
                    Some(&file.name),
                    Some(file.mime.parse().unwrap()),
                );
            } else {
                multipart.add_text(key, value.to_string());
            }
        }
        let mut payload = multipart.prepare().map_err(Into::<std::io::Error>::into)?;
        let mut buf = vec![];
        payload.read_to_end(&mut buf)?;

        let mut headers = Headers::new();
        headers.set(
            "Content-Type",
            &format!("multipart/form-data; boundary={}", payload.boundary()),
        )?;

        request
            .with_method(worker::Method::Post)
            .with_body(Some(worker::js_sys::Uint8Array::from(&buf[..]).into()))
            .with_headers(headers);

        let response = Fetch::Request(Request::new_with_init(
            &format!("{}{}", self.base_url, Method::name()),
            &request,
        )?)
        .send()
        .await?;

        Self::parse_response::<Method>(response).await
    }

    async fn parse_response<Method: TelegramMethod>(
        mut response: Response,
    ) -> Result<Method::Response> {
        let tg_response: ApiResponse<_> = response.json().await?;
        match tg_response {
            ApiResponse::Ok { result } => Ok(result),
            ApiResponse::Err(error) => Err(Error::TelegramError(error)),
        }
    }
}
