use serde::{Deserialize, Serialize};

use crate::file::InputFile;
use crate::message::{Location, Message};
use crate::user::User;
use crate::{JsonMethod, TelegramMethod};

/// This object represents a chat.
#[derive(Deserialize)]
pub struct Chat {
    /// Unique identifier for this chat.
    pub id: i64,
    /// Type of chat.
    #[serde(flatten)]
    pub kind: ChatKind,
    /// Title, for supergroups, channels and group chats
    pub title: Option<String>,
    /// Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    /// First name of the other party in a private chat
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat
    pub last_name: Option<String>,
    /// Chat photo.
    /// Returned only in getChat.
    pub photo: Option<ChatPhoto>,
    /// Bio of the other party in a private chat.
    /// Returned only in getChat.
    pub bio: Option<String>,
    /// Description, for groups, supergroups and channel chats.
    /// Returned only in getChat.
    pub description: Option<String>,
    /// Primary invite link, for groups, supergroups and channel chats.
    /// Returned only in getChat.
    pub invite_link: Option<String>,
    /// The most recent pinned message (by sending date).
    /// Returned only in getChat.
    pub pinned_message: Option<Box<Message>>,
    /// Default chat member permissions, for groups and supergroups.
    /// Returned only in getChat.
    pub permissions: Option<ChatPermissions>,
    /// Default chat member permissions, for groups and supergroups.
    /// Returned only in getChat.
    pub slow_mode_delay: Option<i32>,
    /// The time after which all messages sent to the chat will be automatically deleted; in seconds.
    /// Returned only in getChat.
    pub message_auto_delete_time: Option<i32>,
    /// For supergroups, name of group sticker set.
    /// Returned only in getChat.
    pub sticker_set_name: Option<String>,
    /// True, if the bot can change the group sticker set.
    /// Returned only in getChat.
    pub can_get_sticker_set: Option<bool>,
    /// Unique identifier for the linked chat,
    /// i.e. the discussion group identifier for a channel and vice versa;
    /// for supergroups and channel chats.
    /// Returned only in getChat.
    pub linked_chat_id: Option<i32>,
    /// For supergroups, the location to which the supergroup is connected.
    /// Returned only in getChat.
    pub location: Option<ChatLocation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ChatKind {
    Private,
    Group,
    Supergroup,
    Channel,
}

/// This object represents a chat photo.
#[derive(Deserialize)]
pub struct ChatPhoto {
    /// File identifier of small (160x160) chat photo.
    /// This file_id can be used only for photo download
    /// and only for as long as the photo is not changed.
    pub small_file_id: String,
    /// Unique file identifier of small (160x160) chat photo,
    /// which is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub small_file_unique_id: String,
    /// File identifier of big (640x640) chat photo.
    /// This file_id can be used only for photo download
    /// and only for as long as the photo is not changed.
    pub big_file_id: String,
    /// Unique file identifier of big (640x640) chat photo,
    /// which is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub big_file_unique_id: String,
}

#[derive(Deserialize)]
pub struct ChatLocation {
    /// The location to which the supergroup is connected.
    /// Can't be a live location.
    pub location: Location,
    /// Location address; 1-64 characters, as defined by the chat owner
    pub address: String,
}

/// Describes actions that a non-administrator user is allowed to take in a chat.
#[derive(Serialize, Deserialize, Default)]
pub struct ChatPermissions {
    /// True, if the user is allowed to send text messages, contacts, locations and venues
    pub can_send_messages: Option<bool>,
    /// True, if the user is allowed to send audios, documents,
    /// photos, videos, video notes and voice notes, implies can_send_messages
    pub can_send_media_messages: Option<bool>,
    /// True, if the user is allowed to send polls, implies can_send_messages
    pub can_send_polls: Option<bool>,
    /// True, if the user is allowed to send animations, games, stickers
    /// and use inline bots, implies can_send_media_messages
    pub can_send_other_messages: Option<bool>,
    /// True, if the user is allowed to add web page previews to their messages,
    /// implies can_send_media_messages
    pub can_add_web_page_previews: Option<bool>,
    /// True, if the user is allowed to change the chat title, photo and other settings.
    /// Ignored in public supergroups
    pub can_change_info: Option<bool>,
    /// True, if the user is allowed to invite new users to the chat
    pub can_invite_users: Option<bool>,
    /// True, if the user is allowed to pin messages.
    /// Ignored in public supergroups
    pub can_pin_messages: Option<bool>,
}

impl ChatPermissions {
    /// Create a new ChatPermissions object
    pub fn new() -> Self {
        Default::default()
    }
    /// Set can_send_messages to `true`
    pub fn allow_send_messages(self) -> Self {
        Self {
            can_send_messages: Some(true),
            ..self
        }
    }
    /// Set can_send_media_messages to `true`
    pub fn allow_send_media_messages(self) -> Self {
        Self {
            can_send_media_messages: Some(true),
            ..self
        }
    }
    /// Set can_send_polls to `true`
    pub fn allow_send_polls(self) -> Self {
        Self {
            can_send_polls: Some(true),
            ..self
        }
    }
    /// Set can_send_other_messages to `true`
    pub fn allow_send_other_messages(self) -> Self {
        Self {
            can_send_other_messages: Some(true),
            ..self
        }
    }
    /// Set can_add_web_page_previews to `true`
    pub fn allow_add_web_page_previews(self) -> Self {
        Self {
            can_add_web_page_previews: Some(true),
            ..self
        }
    }
    /// Set can_change_info to `true`
    pub fn allow_change_info(self) -> Self {
        Self {
            can_change_info: Some(true),
            ..self
        }
    }
    /// Set can_invite_users to `true`
    pub fn allow_invite_users(self) -> Self {
        Self {
            can_invite_users: Some(true),
            ..self
        }
    }
    /// Set can_pin_messages to `true`
    pub fn allow_pin_messages(self) -> Self {
        Self {
            can_pin_messages: Some(true),
            ..self
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case", tag = "status")]
pub enum ChatMember {
    /// Represents a [chat member](https://core.telegram.org/bots/api#chatmember)
    /// that owns the chat and has all administrator privileges.
    #[serde(rename = "creator")]
    Owner {
        /// Information about the user
        user: User,
        /// True, if the user's presence in the chat is hidden
        is_anonymous: bool,
        /// Custom title for this user
        custom_title: Option<String>,
    },
    /// Represents a [chat member](https://core.telegram.org/bots/api#chatmember)
    /// that has some additional privileges.
    Administrator {
        /// Information about the user
        user: User,
        /// True, if the bot is allowed to edit administrator privileges of that user
        can_be_edited: bool,
        /// True, if the user's presence in the chat is hidden
        is_anonymous: bool,
        /// True, if the administrator can access the chat event log, chat statistics,
        /// message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode.
        /// Implied by any other administrator privilege
        can_manage_chat: bool,
        /// True, if the administrator can delete messages of other users
        can_delete_messages: bool,
        /// True, if the administrator can manage voice chats
        can_manage_voice_chats: bool,
        /// True, if the administrator can restrict, ban or unban chat members
        can_restrict_members: bool,
        /// True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted,
        /// directly or indirectly (promoted by administrators that were appointed by the user)
        can_promote_members: bool,
        /// True, if the user is allowed to change the chat title, photo and other settings
        can_change_info: bool,
        /// True, if the user is allowed to invite new users to the chat
        can_invite_users: bool,
        /// True, if the administrator can post in the channel; channels only
        can_post_messages: Option<bool>,
        /// True, if the administrator can edit messages of other users and can pin messages; channels only
        can_edit_messages: Option<bool>,
        /// True, if the user is allowed to pin messages; groups and supergroups only
        can_pin_messages: Option<bool>,
        /// Custom title for this user
        custom_title: Option<String>,
    },
    /// Represents a [chat member](https://core.telegram.org/bots/api#chatmember)
    /// that has no additional privileges or restrictions.
    Member {
        /// Information about the user
        user: User,
    },
    /// Represents a [chat member](https://core.telegram.org/bots/api#chatmember)
    /// that is under certain restrictions in the chat. Supergroups only.
    Restricted {
        /// Information about the user
        user: User,
        /// True, if the user is a member of the chat at the moment of the request
        is_member: bool,
        /// True, if the user is allowed to change the chat title, photo and other settings
        can_change_info: bool,
        /// True, if the user is allowed to invite new users to the chat
        can_invite_users: bool,
        /// True, if the user is allowed to pin messages
        can_pin_messages: bool,
        /// True, if the user is allowed to send text messages, contacts, locations and venues
        can_send_messages: bool,
        /// True, if the user is allowed to send audios, documents, photos, videos, video notes and voice notes
        can_send_media_messages: bool,
        /// True, if the user is allowed to send polls
        can_send_polls: bool,
        /// True, if the user is allowed to send animations, games, stickers and use inline bots
        can_send_other_messages: bool,
        /// True, if the user is allowed to add web page previews to their messages
        can_add_web_page_previews: bool,
        /// Date when restrictions will be lifted for this user; unix time.
        /// If 0, then the user is restricted forever
        until_date: u64,
    },
    /// Represents a [chat member](https://core.telegram.org/bots/api#chatmember)
    /// that isn't currently a member of the chat, but may join it themselves.
    Left {
        /// Information about the user
        user: User,
    },
    /// Represents a [chat member](https://core.telegram.org/bots/api#chatmember)
    /// that was banned in the chat and can't return to the chat or view chat messages.
    #[serde(rename = "kicked")]
    Banned {
        /// Information about the user
        user: User,
        /// Date when restrictions will be lifted for this user; unix time.
        /// If 0, then the user is banned forever
        until_date: u64,
    },
}

/// Represents an invite link for a chat.
#[derive(Deserialize)]
pub struct ChatInviteLink {
    /// The invite link.
    /// If the link was created by another chat administrator,
    /// then the second part of the link will be replaced with “…”.
    pub invite_link: String,
    /// Creator of the link
    pub creator: User,
    /// True, if the link is primary
    pub is_primary: bool,
    /// True, if the link is revoked
    pub is_revoked: bool,
    /// Point in time (Unix timestamp) when the link will expire or has been expired
    pub expire_date: Option<u64>,
    /// Maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    pub member_limit: Option<u32>,
}

/// This object represents changes in the status of a chat member.
#[derive(Deserialize)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to
    pub chat: Chat,
    /// Performer of the action, which resulted in the change
    pub from: User,
    /// Date the change was done in Unix time
    pub date: u64,
    /// Previous information about the chat member
    pub old_chat_member: ChatMember,
    /// New information about the chat member
    pub new_chat_member: ChatMember,
    /// Chat invite link, which was used by the user to join the chat;
    /// for joining by invite link events only.
    pub invite_link: Option<ChatInviteLink>,
}

/// Chat identifier
#[derive(Serialize)]
#[serde(untagged)]
pub enum ChatId {
    Id(i64),
    Username(String),
}

impl From<i64> for ChatId {
    fn from(id: i64) -> Self {
        Self::Id(id)
    }
}

impl From<String> for ChatId {
    fn from(username: String) -> Self {
        Self::Username(username)
    }
}

impl From<&str> for ChatId {
    fn from(username: &str) -> Self {
        Self::Username(username.to_string())
    }
}

/// Use this method to ban a user in a group, a supergroup or a channel.
/// In the case of supergroups and channels, the user will not be able to return to the chat
/// on their own using invite links, etc., unless unbanned first.
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// Returns True on success.
#[derive(Serialize)]
pub struct BanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Date when the user will be unbanned, unix time.
    /// If user is banned for more than 366 days or less than 30 seconds from the current time
    /// they are considered to be banned forever.
    /// Applied for supergroups and channels only.
    pub until_date: Option<u64>,
    /// Pass _True_ to delete all messages from the chat for the user that is being removed.
    /// If _False_, the user will be able to see messages in the group that were sent before the user was removed.
    /// Always _True_ for supergroups and channels.
    pub revoke_messages: Option<bool>,
}

impl BanChatMember {
    /// Create a new banChatMember request
    pub fn new(chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
            until_date: None,
            revoke_messages: None,
        }
    }

    /// Set the date at which the user will be unbanned
    pub fn until_date(self, date: u64) -> Self {
        Self {
            until_date: Some(date),
            ..self
        }
    }

    /// Set revoke_messages to `true`
    pub fn revoke_messages(self) -> Self {
        Self {
            revoke_messages: Some(true),
            ..self
        }
    }
}

impl TelegramMethod for BanChatMember {
    type Response = bool;

    fn name() -> &'static str {
        "banChatMember"
    }
}

impl JsonMethod for BanChatMember {}

/// Use this method to unban a previously banned user in a supergroup or channel.
/// The user will **not** return to the group or channel automatically, but will be able to join via link, etc.
/// The bot must be an administrator for this to work.
/// By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it.
/// So if the user is a member of the chat they will also be **removed** from the chat.
/// If you don't want this, use the parameter *only_if_banned*.
/// Returns _True_ on success.
#[derive(Serialize)]
pub struct UnbanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Do nothing if the user is not banned
    pub only_if_banned: Option<bool>,
}

impl UnbanChatMember {
    /// Create a new unbanChatMember request
    pub fn new(chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
            only_if_banned: None,
        }
    }

    /// Set only_if_banned to `true`
    pub fn only_if_banned(self) -> Self {
        Self {
            only_if_banned: Some(true),
            ..self
        }
    }
}

impl TelegramMethod for UnbanChatMember {
    type Response = bool;

    fn name() -> &'static str {
        "unbanChatMember"
    }
}

impl JsonMethod for UnbanChatMember {}

/// Use this method to restrict a user in a supergroup.
/// The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights.
/// Pass *True* for all permissions to lift restrictions from a user.
/// Returns *True* on success.
#[derive(Serialize)]
pub struct RestrictChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// A JSON-serialized object for new user permissions
    pub permissions: ChatPermissions,
    /// Date when restrictions will be lifted for the user, unix time.
    /// If user is restricted for more than 366 days or less than 30 seconds from the current time,
    /// they are considered to be restricted forever
    pub until_date: Option<u64>,
}

impl RestrictChatMember {
    /// Create a new restrictChatMember request
    pub fn new(chat_id: impl Into<ChatId>, user_id: i64, permissions: ChatPermissions) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
            permissions,
            until_date: None,
        }
    }

    /// Set the date at which the restriction wil be lifted
    pub fn until_date(self, date: u64) -> Self {
        Self {
            until_date: Some(date),
            ..self
        }
    }
}

impl TelegramMethod for RestrictChatMember {
    type Response = bool;

    fn name() -> &'static str {
        "restrictChatMember"
    }
}

impl JsonMethod for RestrictChatMember {}

/// Use this method to promote or demote a user in a supergroup or a channel.
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// Pass _False_ for all boolean parameters to demote a user.
/// Returns _True_ on success.
#[derive(Serialize)]
pub struct PromoteChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Pass _True_, if the administrator's presence in the chat is hidden
    pub is_anonymous: Option<bool>,
    /// Pass _True_, if the administrator can access the chat event log, chat statistics, message statistics in channels,
    /// see channel members, see anonymous administrators in supergroups and ignore slow mode.
    /// Implied by any other administrator privilege
    pub can_manage_chat: Option<bool>,
    /// Pass _True_, if the administrator can delete messages of other users
    pub can_delete_messages: Option<bool>,
    /// Pass _True_, if the administrator can manage voice chats
    pub can_manage_voice_chats: Option<bool>,
    /// Pass _True_, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: Option<bool>,
    /// Pass _True_, if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted,
    /// directly or indirectly (promoted by administrators that were appointed by him)
    pub can_promote_members: Option<bool>,
    /// Pass _True_, if the administrator can change chat title, photo and other settings
    pub can_change_info: Option<bool>,
    /// Pass _True_, if the administrator can invite new users to the chat
    pub can_invite_users: Option<bool>,
    /// Pass _True_, if the administrator can create channel posts, channels only
    pub can_post_messages: Option<bool>,
    /// Pass _True_, if the administrator can edit messages of other users and can pin messages, channels only
    pub can_edit_messages: Option<bool>,
    /// Pass _True_, if the administrator can pin messages, supergroups only
    pub can_pin_messages: Option<bool>,
}

impl PromoteChatMember {
    /// Create a new promoteChatMember request
    pub fn new(chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
            is_anonymous: None,
            can_manage_chat: None,
            can_delete_messages: None,
            can_manage_voice_chats: None,
            can_restrict_members: None,
            can_promote_members: None,
            can_change_info: None,
            can_invite_users: None,
            can_post_messages: None,
            can_edit_messages: None,
            can_pin_messages: None,
        }
    }

    /// Create a new promoteChatMember request that demotes the user
    ///
    /// It creates a new promoteChatMember request with all options disabled.
    pub fn demote(chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
            is_anonymous: Some(false),
            can_manage_chat: Some(false),
            can_delete_messages: Some(false),
            can_manage_voice_chats: Some(false),
            can_restrict_members: Some(false),
            can_promote_members: Some(false),
            can_change_info: Some(false),
            can_invite_users: Some(false),
            can_post_messages: Some(false),
            can_edit_messages: Some(false),
            can_pin_messages: Some(false),
        }
    }

    /// Set `is_anonymous` to `true`
    pub fn anonymous(self) -> Self {
        Self {
            is_anonymous: Some(true),
            ..self
        }
    }

    /// Set `can_manage_chat` to `true`
    pub fn allow_manage_chat(self) -> Self {
        Self {
            can_manage_chat: Some(true),
            ..self
        }
    }

    /// Set `can_delete_messages` to `true`
    pub fn allow_delete_messages(self) -> Self {
        Self {
            can_delete_messages: Some(true),
            ..self
        }
    }

    /// Set `can_manage_voice_chats` to `true`
    pub fn allow_manage_voice_chats(self) -> Self {
        Self {
            can_manage_voice_chats: Some(true),
            ..self
        }
    }

    /// Set `can_restrict_members` to `true`
    pub fn allow_restrict_members(self) -> Self {
        Self {
            can_restrict_members: Some(true),
            ..self
        }
    }

    /// Set `can_promote_members` to `true`
    pub fn allow_promote_members(self) -> Self {
        Self {
            can_promote_members: Some(true),
            ..self
        }
    }

    /// Set `can_change_info` to `true`
    pub fn allow_change_info(self) -> Self {
        Self {
            can_change_info: Some(true),
            ..self
        }
    }

    /// Set `can_invite_users` to `true`
    pub fn allow_invite_users(self) -> Self {
        Self {
            can_invite_users: Some(true),
            ..self
        }
    }

    /// Set `can_post_messages` to `true`
    pub fn allow_post_messages(self) -> Self {
        Self {
            can_post_messages: Some(true),
            ..self
        }
    }

    /// Set `can_edit_messages` to `true`
    pub fn allow_edit_messages(self) -> Self {
        Self {
            can_edit_messages: Some(true),
            ..self
        }
    }

    /// Set `can_pin_messages` to `true`
    pub fn allow_pin_messages(self) -> Self {
        Self {
            can_pin_messages: Some(true),
            ..self
        }
    }
}

impl TelegramMethod for PromoteChatMember {
    type Response = bool;

    fn name() -> &'static str {
        "promoteChatMember"
    }
}

impl JsonMethod for PromoteChatMember {}

/// Use this method to set a custom title for an administrator in a supergroup promoted by the bot.
/// Returns _True_ on success.
#[derive(Serialize)]
pub struct SetChatAdministratorCustomTitle {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// New custom title for the administrator; 0-16 characters, emoji are not allowed
    pub custom_title: String,
}

impl SetChatAdministratorCustomTitle {
    /// Create a new setChatAdministratorCustomTitle request
    pub fn new(chat_id: impl Into<ChatId>, user_id: i64, custom_title: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
            custom_title: custom_title.into(),
        }
    }
}

impl TelegramMethod for SetChatAdministratorCustomTitle {
    type Response = bool;

    fn name() -> &'static str {
        "setChatAdministratorCustomTitle"
    }
}

impl JsonMethod for SetChatAdministratorCustomTitle {}

/// Use this method to set default chat permissions for all members.
/// The bot must be an administrator in the group or a supergroup for this to work and must have the *can_restrict_members* administrator rights.
/// Returns _True_ on success.
#[derive(Serialize)]
pub struct SetChatPermissions {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`)
    pub chat_id: ChatId,
    /// A JSON-serialized object for new user permissions
    pub permissions: ChatPermissions,
}

impl SetChatPermissions {
    /// Create a new setChatPermissions request
    pub fn new(chat_id: impl Into<ChatId>, permissions: ChatPermissions) -> Self {
        Self {
            chat_id: chat_id.into(),
            permissions,
        }
    }
}

impl TelegramMethod for SetChatPermissions {
    type Response = bool;

    fn name() -> &'static str {
        "setChatPermissions"
    }
}

impl JsonMethod for SetChatPermissions {}

/// Use this method to generate a new primary invite link for a chat;
/// any previously generated primary link is revoked.
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// Returns the new invite link as _String_ on success.
///
/// Note: Each administrator in a chat generates their own invite links.
/// Bots can't use invite links generated by other administrators.
/// If you want your bot to work with invite links,
/// it will need to generate its own link using [exportChatInviteLink](https://core.telegram.org/bots/api#exportchatinvitelink)
/// or by calling the [getChat](https://core.telegram.org/bots/api#getchat) method.
/// If your bot needs to generate a new primary invite link replacing its previous one,
/// use [exportChatInviteLink](https://core.telegram.org/bots/api#exportchatinvitelink) again.
#[derive(Serialize)]
pub struct ExportChatInviteLink {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`)
    pub chat_id: ChatId,
}

impl ExportChatInviteLink {
    /// Create a new exportChatInviteLink request
    pub fn new(chat_id: impl Into<ChatId>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }
}

impl TelegramMethod for ExportChatInviteLink {
    type Response = String;

    fn name() -> &'static str {
        "exportChatInviteLink"
    }
}

impl JsonMethod for ExportChatInviteLink {}

/// Use this method to revoke an invite link created by the bot.
/// If the primary link is revoked, a new link is automatically generated.
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// Returns the revoked invite link as [ChatInviteLink](https://core.telegram.org/bots/api#chatinvitelink) object.
#[derive(Serialize)]
pub struct RevokeChatInviteLink {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`)
    pub chat_id: ChatId,
    /// The invite link to revoke
    pub invite_link: String,
}

impl RevokeChatInviteLink {
    /// Create a new revokeChatInviteLink object
    pub fn new(chat_id: impl Into<ChatId>, invite_link: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
        }
    }
}

impl TelegramMethod for RevokeChatInviteLink {
    type Response = ChatInviteLink;

    fn name() -> &'static str {
        "revokeChatInviteLink"
    }
}

impl JsonMethod for RevokeChatInviteLink {}

/// Use this method to approve a chat join request.
/// The bot must be an administrator in the chat for this to work and must have the *can_invite_users* administrator right.
/// Returns _True_ on success.
#[derive(Serialize)]
pub struct ApproveChatJoinRequest {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}

impl ApproveChatJoinRequest {
    /// Create a new approveChatJoinRequest request
    pub fn new(chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl TelegramMethod for ApproveChatJoinRequest {
    type Response = bool;

    fn name() -> &'static str {
        "approveChatJoinRequest"
    }
}

impl JsonMethod for ApproveChatJoinRequest {}

/// Use this method to decline a chat join request.
/// The bot must be an administrator in the chat for this to work and must have the *can_invite_users* administrator right.
/// Returns _True_ on success.
#[derive(Serialize)]
pub struct DeclineChatJoinRequest {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}

impl DeclineChatJoinRequest {
    /// Create a new declineChatJoinRequest request
    pub fn new(chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl TelegramMethod for DeclineChatJoinRequest {
    type Response = bool;

    fn name() -> &'static str {
        "declineChatJoinRequest"
    }
}

impl JsonMethod for DeclineChatJoinRequest {}

/// Use this method to set a new profile photo for the chat.
/// Photos can't be changed for private chats.
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// Returns _True_ on success.
#[derive(Serialize)]
pub struct SetChatPhoto {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`)
    pub chat_id: ChatId,
    /// New chat photo, uploaded using multipart/form-data
    pub photo: InputFile,
}

impl SetChatPhoto {
    /// Create a new setChatPhoto request
    pub fn new(chat_id: impl Into<ChatId>, photo: InputFile) -> Self {
        Self {
            chat_id: chat_id.into(),
            photo,
        }
    }
}

impl TelegramMethod for SetChatPhoto {
    type Response = bool;

    fn name() -> &'static str {
        "setChatPhoto"
    }
}

impl JsonMethod for SetChatPhoto {}

/// Use this method to delete a chat photo.
/// Photos can't be changed for private chats.
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// Returns _True_ on success.
#[derive(Serialize)]
pub struct DeleteChatPhoto {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`)
    pub chat_id: ChatId,
}

impl DeleteChatPhoto {
    /// Create a new deleteChatPhoto request
    pub fn new(chat_id: impl Into<ChatId>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }
}

impl TelegramMethod for DeleteChatPhoto {
    type Response = bool;

    fn name() -> &'static str {
        "deleteChatPhoto"
    }
}

impl JsonMethod for DeleteChatPhoto {}

/// Use this method to change the title of a chat.
/// Titles can't be changed for private chats.
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// Returns _True_ on success.
#[derive(Serialize)]
pub struct SetChatTitle {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`)
    pub chat_id: ChatId,
    /// New chat title, 1-255 characters
    pub title: String,
}

impl SetChatTitle {
    /// Create a new setChatTitle request
    pub fn new(chat_id: impl Into<ChatId>, title: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            title: title.into(),
        }
    }
}

impl TelegramMethod for SetChatTitle {
    type Response = bool;

    fn name() -> &'static str {
        "setChatTitle"
    }
}

impl JsonMethod for SetChatTitle {}

/// Use this method to change the description of a group, a supergroup or a channel.
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// Returns _True_ on success.
#[derive(Serialize)]
pub struct SetChatDescription {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`)
    pub chat_id: ChatId,
    /// New chat description, 0-255 characters
    pub description: Option<String>,
}

impl SetChatDescription {
    /// Create a new setChatDescription request which empties the description
    pub fn new_empty(chat_id: impl Into<ChatId>) -> Self {
        Self {
            chat_id: chat_id.into(),
            description: None,
        }
    }

    /// Create a new setChatDescription request with description
    pub fn new(chat_id: impl Into<ChatId>, description: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            description: Some(description.into()),
        }
    }
}

impl TelegramMethod for SetChatDescription {
    type Response = bool;

    fn name() -> &'static str {
        "setChatDescription"
    }
}

/// Use this method to add a message to the list of pinned messages in a chat.
/// If the chat is not a private chat, the bot must be an administrator in the chat for this to work
/// and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel.
/// Returns _True_ on success.
#[derive(Serialize)]
pub struct PinChatMessage {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Identifier of a message to pin
    pub message_id: i64,
    /// Pass True, if it is not necessary to send a notification to all chat members about the new pinned message.
    /// Notifications are always disabled in channels and private chats.
    pub disable_notification: Option<bool>,
}

impl PinChatMessage {
    /// Create a new pinChatMessage request
    pub fn new(chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id,
            disable_notification: None,
        }
    }

    /// Disable notification about the new pinned message
    pub fn disable_notification(self) -> Self {
        Self {
            disable_notification: Some(true),
            ..self
        }
    }
}

impl TelegramMethod for PinChatMessage {
    type Response = bool;

    fn name() -> &'static str {
        "pinChatMessage"
    }
}

impl JsonMethod for PinChatMessage {}

/// Use this method to remove a message from the list of pinned messages in a chat.
/// If the chat is not a private chat, the bot must be an administrator in the chat for this to work
/// and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel.
/// Returns _True_ on success.
#[derive(Serialize)]
pub struct UnpinChatMessage {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Identifier of a message to unpin. If not specified, the most recent pinned message (by sending date) will be unpinned.
    pub message_id: Option<i64>,
}

impl UnpinChatMessage {
    /// Create a new unpinChatMessage request that unpins the most recent pinned message
    pub fn new_recent(chat_id: impl Into<ChatId>) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id: None,
        }
    }

    /// Create a new unpinChatMessage request
    pub fn new(chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id: Some(message_id),
        }
    }
}

impl TelegramMethod for UnpinChatMessage {
    type Response = bool;

    fn name() -> &'static str {
        "unpinChatMessage"
    }
}

impl JsonMethod for UnpinChatMessage {}

/// Use this method to clear the list of pinned messages in a chat.
/// If the chat is not a private chat, the bot must be an administrator in the chat for this to work
/// and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel.
/// Returns _True_ on success.
#[derive(Serialize)]
pub struct UnpinAllChatMessages {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
}

impl UnpinAllChatMessages {
    /// Create a new unpinAllChatMessages request
    pub fn new(chat_id: impl Into<ChatId>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }
}

impl TelegramMethod for UnpinAllChatMessages {
    type Response = bool;

    fn name() -> &'static str {
        "unpinAllChatMessages"
    }
}

impl JsonMethod for UnpinAllChatMessages {}

/// Use this method for your bot to leave a group, supergroup or channel. Returns _True_ on success.
#[derive(Serialize)]
pub struct LeaveChat {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
}

impl LeaveChat {
    /// Create a new leaveChat request
    pub fn new(chat_id: impl Into<ChatId>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }
}

impl TelegramMethod for LeaveChat {
    type Response = bool;

    fn name() -> &'static str {
        "leaveChat"
    }
}

impl JsonMethod for LeaveChat {}

/// Use this method to get up to date information about the chat
/// (current name of the user for one-on-one conversations, current username of a user, group or channel, etc.).
/// Returns a Chat object on success.
#[derive(Serialize)]
pub struct GetChat {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
}

impl GetChat {
    /// Create a new getChat request
    pub fn new(chat_id: impl Into<ChatId>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }
}

impl TelegramMethod for GetChat {
    type Response = Chat;

    fn name() -> &'static str {
        "getChat"
    }
}

impl JsonMethod for GetChat {}

/// Use this method to get a list of administrators in a chat.
/// On success, returns an Array of [ChatMember](https://core.telegram.org/bots/api#chatmember) objects
/// that contains information about all chat administrators except other bots.
/// If the chat is a group or a supergroup and no administrators were appointed, only the creator will be returned.
#[derive(Serialize)]
pub struct GetChatAdministrators {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
}

impl GetChatAdministrators {
    /// Create a new getChatAdministrators request
    pub fn new(chat_id: impl Into<ChatId>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }
}

impl TelegramMethod for GetChatAdministrators {
    type Response = Vec<ChatMember>;

    fn name() -> &'static str {
        "getChatAdministrators"
    }
}

impl JsonMethod for GetChatAdministrators {}

/// Use this method to get the number of members in a chat. Returns _Int_ on success.
#[derive(Serialize)]
pub struct GetChatMemberCount {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
}

impl GetChatMemberCount {
    /// Create a new getChatMemberCount request
    pub fn new(chat_id: impl Into<ChatId>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }
}

impl TelegramMethod for GetChatMemberCount {
    type Response = u32;

    fn name() -> &'static str {
        "getChatMemberCount"
    }
}

impl JsonMethod for GetChatMemberCount {}

/// Use this method to get information about a member of a chat.
/// Returns a [ChatMember](https://core.telegram.org/bots/api#chatmember) object on success.
#[derive(Serialize)]
pub struct GetChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}

impl GetChatMember {
    /// Create a new getChatMember request
    pub fn new(chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl TelegramMethod for GetChatMember {
    type Response = ChatMember;

    fn name() -> &'static str {
        "getChatMember"
    }
}

/// Use this method to set a new group sticker set for a supergroup.
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// Use the field *can_set_sticker_set* optionally returned in [getChat](https://core.telegram.org/bots/api#getchat) requests to check if the bot can use this method.
/// Returns _True_ on success.
#[derive(Serialize)]
pub struct SetChatStickerSet {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Name of the sticker set to be set as the group sticker set
    pub sticker_set_name: String,
}

impl SetChatStickerSet {
    /// Create a new setChatStickerSet request
    pub fn new(chat_id: impl Into<ChatId>, sticker_set_name: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            sticker_set_name: sticker_set_name.into(),
        }
    }
}

impl TelegramMethod for SetChatStickerSet {
    type Response = bool;

    fn name() -> &'static str {
        "setChatStickerSet"
    }
}

impl JsonMethod for SetChatStickerSet {}

/// Use this method to delete a group sticker set from a supergroup.
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// Use the field *can_set_sticker_set* optionally returned in [getChat](https://core.telegram.org/bots/api#getchat) requests to check if the bot can use this method.
/// Returns _True_ on success.
#[derive(Serialize)]
pub struct DeleteChatStickerSet {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
}

impl DeleteChatStickerSet {
    /// Create a new deleteChatStickerSet request
    pub fn new(chat_id: impl Into<ChatId>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }
}

impl TelegramMethod for DeleteChatStickerSet {
    type Response = bool;

    fn name() -> &'static str {
        "deleteChatStickerSet"
    }
}

impl JsonMethod for DeleteChatStickerSet {}
