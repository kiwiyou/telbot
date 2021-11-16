//! `telbot-types` provides a set of types compatible with [Telegram bot API](https://core.telegram.org/bots/api).
//!
//! # Backends
//!
//! If you want to send requests or receive data from the API server, please refer to any of the following crates:
//!
//! - `telbot-cf-worker` for cloudflare workers backend
//! - `telbot-ureq` for `ureq` backend
//! - `telbot-reqwest` for `reqwest` backend
//!
//! ## Extending backends
//! 
//! Every API request type implements either [`JsonMethod`] or [`FileMethod`],
//! representing those should be serialized into JSON format and multipart format, respectively.
//! Your backend should take these two types of request and deserialize the response body into [`ApiResponse<T>`].
//! Then you can take the actual response `T` from `ApiResponse<T>`.

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

/// Base trait for telegram method.
pub trait TelegramMethod {
    /// Method response type
    type Response: DeserializeOwned;

    /// Gets the name of the method.
    /// 
    /// Used in request URL, like `https://api.telegram.org/bot<BOT TOKEN>/<METHOD NAME>`.
    fn name() -> &'static str;
}

/// Methods that should be sent in JSON format.
pub trait JsonMethod: TelegramMethod + Serialize {}

/// Methods that should be sent in multipart or JSON format.
pub trait FileMethod: TelegramMethod + Serialize {
    /// Gets a (name, value) map of file-type fields.
    fn files(&self) -> Option<HashMap<&str, &InputFile>>;
}

/// Telegram API response.
/// 
/// Response body should be deserialized into [`ApiResponse<T>`] to handle error correctly.
/// On a successful request, the response value will be in the `result` field.
/// On request failure, the error value will be in the `Err` variant with bad HTTP status code.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T: DeserializeOwned> {
    /// Represents a successful request.
    Ok {
        /// Value returned from the request.
        #[serde(bound(deserialize = "T: DeserializeOwned"))]
        result: T,
    },
    /// Represents a failed request.
    Err(TelegramError),
}

/// Error from Telegram API server.
#[derive(Debug, Deserialize)]
pub struct TelegramError {
    /// Cause of the error.
    pub description: String,
}
