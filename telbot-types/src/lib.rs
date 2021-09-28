use std::collections::HashMap;

use file::InputFile;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub mod bot;
pub mod chat;
pub mod file;
pub mod markup;
pub mod message;
pub mod payment;
pub mod query;
pub mod sticker;
pub mod update;
pub mod user;
pub mod webhook;

pub trait TelegramMethod {
    type Response: DeserializeOwned;
    fn name() -> &'static str;
}

/// Methods that should be sent in JSON format
pub trait JsonMethod: TelegramMethod + Serialize {}

/// Methods that should be sent in multipart or JSON format
pub trait FileMethod: TelegramMethod + Serialize {
    fn files(&self) -> Option<HashMap<&str, &InputFile>>;
}

/// Api response
#[derive(Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T: DeserializeOwned> {
    Ok {
        #[serde(bound(deserialize = "T: DeserializeOwned"))]
        result: T,
    },
    Err(TelegramError),
}

#[derive(Debug, Deserialize)]
pub struct TelegramError {
    pub description: String,
}
