use serde::Serialize;
pub use telbot_types as types;
use telbot_types::{ApiResponse, FileMethod, JsonMethod, TelegramError, TelegramMethod};
use worker::wasm_bindgen::JsValue;
use worker::{Fetch, Headers, Request, RequestInit};

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
}

impl<E> From<E> for Error
where
    worker::Error: From<E>,
{
    fn from(error: E) -> Self {
        Self::Worker(error.into())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl Api {
    /// Send a JSON-serializable API request
    pub async fn send_json<Method: JsonMethod>(&self, method: &Method) -> Result<Method::Response> {
        self.send_json_impl(method).await
    }

    /// Send a JSON-serializable API request with files.
    /// **Note:** multipart data is not supported.
    pub async fn send_file<Method: FileMethod>(&self, method: &Method) -> Result<Method::Response> {
        self.send_json_impl(method).await
    }

    async fn send_json_impl<Method: TelegramMethod + Serialize>(
        &self,
        method: &Method,
    ) -> Result<Method::Response> {
        let mut headers = Headers::new();
        headers.set("Content-Type", "application/json")?;
        let mut request = RequestInit::new();
        let payload = serde_json::to_string(&method)?;
        request
            .with_method(worker::Method::Post)
            .with_body(Some(JsValue::from_str(&payload)))
            .with_headers(headers);

        let mut response = Fetch::Request(Request::new_with_init(
            &format!("{}{}", self.base_url, Method::name()),
            &request,
        )?)
        .send()
        .await?;

        let tg_response: ApiResponse<_> = response.json().await?;
        match tg_response {
            ApiResponse::Ok { result } => Ok(result),
            ApiResponse::Err(error) => Err(Error::TelegramError(error)),
        }
    }
}
