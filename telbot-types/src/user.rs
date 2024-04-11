use serde::{Deserialize, Serialize};

use crate::chat::{
    ApproveChatJoinRequest, BanChatMember, ChatId, ChatPermissions, DeclineChatJoinRequest,
    GetChatMember, PromoteChatMember, RestrictChatMember, SetChatAdministratorCustomTitle,
    UnbanChatMember,
};
use crate::file::PhotoSize;
use crate::{JsonMethod, TelegramMethod};

/// A Telegram user or bot.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#user)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for this user or bot.
    pub id: i64,
    /// `true`, if this user is a bot.
    pub is_bot: bool,
    /// User's or bot's first name.
    pub first_name: String,
    /// User's or bot's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// User's or bot's username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// True, if the bot can be invited to groups.
    /// Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<bool>,
    /// True, if [privacy mode](https://core.telegram.org/bots#privacy-mode) is disabled for the bot.
    /// Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<bool>,
    /// True, if the bot supports inline queries.
    /// Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<bool>,
}

impl User {
    /// Creates a new [`GetUserProfilePhotos`] request that gets the profile photos of this user.
    pub fn get_profile_photos(&self) -> GetUserProfilePhotos {
        GetUserProfilePhotos::new(self.id)
    }

    /// Creates a new [`BanChatMember`] request that bans this user from the given chat.
    pub fn ban_from(&self, chat_id: impl Into<ChatId>) -> BanChatMember {
        BanChatMember::new(chat_id, self.id)
    }

    /// Creates a new [`UnbanChatMember`] request that unbans this user from the given chat.
    pub fn unban_from(&self, chat_id: impl Into<ChatId>) -> UnbanChatMember {
        UnbanChatMember::new(chat_id, self.id)
    }

    /// Creates a new [`RestrictChatMember`] request that restricts this user from the given chat with the given permissions.
    pub fn restrict_from(
        &self,
        chat_id: impl Into<ChatId>,
        permissions: ChatPermissions,
    ) -> RestrictChatMember {
        RestrictChatMember::new(chat_id, self.id, permissions)
    }

    /// Creates a new [`PromoteChatMember`] request that promote this user from the given chat.
    pub fn promote_from(&self, chat_id: impl Into<ChatId>) -> PromoteChatMember {
        PromoteChatMember::new(chat_id, self.id)
    }

    /// Creates a new [`SetChatAdministratorCustomTitle`] request that sets this user's administrator title from the given chat to the given title.
    pub fn set_administrator_title_from(
        &self,
        chat_id: impl Into<ChatId>,
        custom_title: impl Into<String>,
    ) -> SetChatAdministratorCustomTitle {
        SetChatAdministratorCustomTitle::new(chat_id, self.id, custom_title)
    }

    /// Creates a new [`ApproveChatJoinRequest`] request that approves this user to join the given chat.
    pub fn approve_join_to(&self, chat_id: impl Into<ChatId>) -> ApproveChatJoinRequest {
        ApproveChatJoinRequest::new(chat_id, self.id)
    }

    /// Creates a new [`DeclineChatJoinRequest`] request that declines this user from joining the given chat.
    pub fn decline_join_to(&self, chat_id: impl Into<ChatId>) -> DeclineChatJoinRequest {
        DeclineChatJoinRequest::new(chat_id, self.id)
    }

    /// Creates a new [`GetChatMember`] request that gets details of this user in the given chat.
    pub fn get_member_from(&self, chat_id: impl Into<ChatId>) -> GetChatMember {
        GetChatMember::new(chat_id, self.id)
    }
}

/// A user's profile pictures.
#[derive(Debug, Deserialize)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has.
    pub total_count: usize,
    /// Requested profile pictures (in up to 4 sizes each).
    pub photos: Vec<Vec<PhotoSize>>,
}

/// Gets a list of profile pictures for a user.
///
/// Returns a [`UserProfilePhotos`] object.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#getuserprofilephotos)
#[derive(Clone, Serialize)]
pub struct GetUserProfilePhotos {
    /// Unique identifier of the target user.
    user_id: i64,
    /// Sequential number of the first photo to be returned. By default, all photos are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    /// Limits the number of photos to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl GetUserProfilePhotos {
    /// Creates a new [`GetUserProfilePhotos`] request that gets profile photos of the given user.
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            offset: None,
            limit: None,
        }
    }
    /// Sets offset.
    pub fn with_offset(self, offset: u32) -> Self {
        Self {
            offset: Some(offset),
            ..self
        }
    }
    /// Sets limit.
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
