use serde::Deserialize;

use crate::message::{Location, Message};
use crate::user::User;

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
#[derive(Deserialize)]
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
    pub invite_link: ChatInviteLink,
}

/// Chat identifier
pub enum ChatId {
    Id(i64),
    Username(String),
}
