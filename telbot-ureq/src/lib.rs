pub mod polling;

use std::io;

use multipart::client::lazy::Multipart;
pub use telbot_types as types;
use telbot_types::{ApiResponse, FileMethod, JsonMethod, TelegramError};
use types::TelegramMethod;
use ureq::Response;

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
    Ureq(ureq::Transport),
    Serde(serde_json::Error),
    Io(std::io::Error),
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
        let response = ureq::post(&format!("{}{}", self.base_url, Method::name())).send_json(value);
        Self::parse_response::<Method>(response)
    }

    /// Send a JSON-serializable API request with files.
    pub fn send_file<Method: FileMethod>(&self, method: &Method) -> Result<Method::Response> {
        let value = serde_json::to_value(method)?;
        let files = method.files();
        let mut multipart = Multipart::new();
        for (key, value) in value.as_object().unwrap().iter() {
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

        let prepared = multipart.prepare().map_err(Into::<io::Error>::into)?;
        let response = ureq::post(&format!("{}{}", self.base_url, Method::name()))
            .set(
                "Content-Type",
                &format!("multipart/form-data; boundary={}", prepared.boundary()),
            )
            .send(prepared);
        Self::parse_response::<Method>(response)
    }

    fn parse_response<Method: TelegramMethod>(
        response: std::result::Result<Response, ureq::Error>,
    ) -> Result<Method::Response> {
        let response = match response {
            Ok(response) => response,
            Err(ureq::Error::Status(_, response)) => response,
            Err(ureq::Error::Transport(e)) => return Err(Error::Ureq(e)),
        };

        let tg_response: ApiResponse<_> = response.into_json()?;
        match tg_response {
            ApiResponse::Ok { result } => Ok(result),
            ApiResponse::Err(error) => Err(Error::TelegramError(error)),
        }
    }
}
