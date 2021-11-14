use reqwest::{
    multipart::{Form, Part},
    Client, Response,
};
pub use telbot_types as types;
use types::{ApiResponse, FileMethod, JsonMethod, TelegramError, TelegramMethod};

#[derive(Clone)]
pub struct Api {
    base_url: String,
    client: Client,
}

#[derive(Debug)]
pub enum Error {
    TelegramError(TelegramError),
    Reqwest(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl Api {
    pub fn new(token: impl AsRef<str>) -> Self {
        Self {
            base_url: format!("https://api.telegram.org/bot{}/", token.as_ref()),
            client: Client::new(),
        }
    }

    pub async fn send_json<Method: JsonMethod>(&self, method: &Method) -> Result<Method::Response> {
        let url = format!("{}{}", self.base_url, Method::name());
        let response = self.client.post(url).json(method).send().await?;
        Self::parse_response::<Method>(response).await
    }

    pub async fn send_file<Method: FileMethod>(&self, method: &Method) -> Result<Method::Response> {
        let url = format!("{}{}", self.base_url, Method::name());
        let files = method.files();
        let serialized = serde_json::to_value(method).unwrap();

        let mut form = Form::new();
        for (key, value) in serialized.as_object().unwrap() {
            if let Some(file) = files.as_ref().and_then(|map| map.get(key.as_str())) {
                form = form.part(
                    key.to_string(),
                    Part::bytes(file.data.clone())
                        .file_name(file.name.clone())
                        .mime_str(&file.mime)
                        .unwrap(),
                );
            } else if let Some(value) = value.as_str() {
                form = form.text(key.to_string(), value.to_string());
            } else {
                form = form.text(key.to_string(), value.to_string());
            }
        }

        let response = self.client.post(url).multipart(form).send().await?;

        Self::parse_response::<Method>(response).await
    }

    async fn parse_response<Method: TelegramMethod>(
        response: Response,
    ) -> Result<Method::Response> {
        let tg_response: ApiResponse<_> = response.json().await?;
        match tg_response {
            ApiResponse::Ok { result } => Ok(result),
            ApiResponse::Err(error) => Err(Error::TelegramError(error)),
        }
    }
}
