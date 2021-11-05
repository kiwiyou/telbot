use multipart::client::lazy::Multipart;
pub use telbot_types as types;
use telbot_types::{ApiResponse, FileMethod, JsonMethod, TelegramError};

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
    Ureq(ureq::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}

impl From<ureq::Error> for Error {
    fn from(error: ureq::Error) -> Self {
        Self::Ureq(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Serde(error)
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
    pub fn send_json<Method: JsonMethod>(&self, method: &Method) -> Result<Method::Response> {
        let value = serde_json::to_value(method)?;
        let response =
            ureq::post(&format!("{}{}", self.base_url, Method::name())).send_json(value)?;

        let tg_response: ApiResponse<_> = response.into_json()?;
        match tg_response {
            ApiResponse::Ok { result } => Ok(result),
            ApiResponse::Err(error) => Err(Error::TelegramError(error)),
        }
    }

    /// Send a JSON-serializable API request with files.
    pub async fn send_file<Method: FileMethod>(&self, method: &Method) -> Result<Method::Response> {
        let value = serde_json::to_value(method)?;
        let files = method.files();
        let mut multipart = Multipart::new();
    }
}
