use serde::{Deserialize, Serialize};

use crate::file::PhotoSize;
use crate::{JsonMethod, TelegramMethod};

/// This object represents a Telegram user or bot.
#[derive(Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for this user or bot.
    pub id: i64,
    /// True, if this user is a bot
    pub is_bot: bool,
    /// User's or bot's first name
    pub first_name: String,
    /// User's or bot's last name
    pub last_name: Option<String>,
    /// User's or bot's username
    pub username: Option<String>,
    /// [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language
    pub language_code: Option<String>,
    /// True, if the bot can be invited to groups.
    /// Returned only in getMe.
    pub can_join_groups: Option<bool>,
    /// True, if [privacy mode](https://core.telegram.org/bots#privacy-mode) is disabled for the bot.
    /// Returned only in getMe.
    pub can_read_all_group_messages: Option<bool>,
    /// True, if the bot supports inline queries.
    /// Returned only in getMe.
    pub supports_inline_queries: Option<bool>,
}

/// This object represent a user's profile pictures.
#[derive(Deserialize)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub total_count: usize,
    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Vec<Vec<PhotoSize>>,
}

/// Use this method to get a list of profile pictures for a user.
/// Returns a [UserProfilePhotos] object.
#[derive(Serialize)]
pub struct GetUserProfilePhotos {
    /// Unique identifier of the target user
    user_id: i64,
    /// Sequential number of the first photo to be returned. By default, all photos are returned.
    offset: Option<u32>,
    /// Limits the number of photos to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    limit: Option<u32>,
}

impl GetUserProfilePhotos {
    /// Create a new getUserProfilePhotos request
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            offset: None,
            limit: None,
        }
    }
    /// Set offset
    pub fn with_offset(self, offset: u32) -> Self {
        Self {
            offset: Some(offset),
            ..self
        }
    }
    /// Set limit
    pub fn with_limit(self, limit: u32) -> Self {
        Self {
            limit: Some(limit),
            ..self
        }
    }
}

impl TelegramMethod for GetUserProfilePhotos {
    type Response = UserProfilePhotos;

    fn name() -> &'static str {
        "getUserProfilePhotos"
    }
}

impl JsonMethod for GetUserProfilePhotos {}
