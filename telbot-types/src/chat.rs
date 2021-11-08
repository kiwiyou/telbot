use serde::{Deserialize, Serialize};

use crate::file::InputFileVariant;
use crate::message::{
    ChatActionKind, Location, Message, SendAnimation, SendAudio, SendChatAction, SendContact,
    SendDice, SendDocument, SendLocation, SendMediaGroup, SendMessage, SendPhoto, SendPoll,
    SendVenue, SendVideo, SendVideoNote, SendVoice,
};
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

impl Chat {
    pub fn send_animation(&self, animation: impl Into<InputFileVariant>) -> SendAnimation {
        SendAnimation::new(self.id, animation)
    }

    pub fn send_audio(&self, audio: impl Into<InputFileVariant>) -> SendAudio {
        SendAudio::new(self.id, audio)
    }

    pub fn send_chat_action(&self, action: ChatActionKind) -> SendChatAction {
        SendChatAction::new(self.id, action)
    }

    pub fn send_contact(
        &self,
        phone_number: impl Into<String>,
        first_name: impl Into<String>,
    ) -> SendContact {
        SendContact::new(self.id, phone_number, first_name)
    }

    pub fn send_dice(&self) -> SendDice {
        SendDice::new(self.id)
    }

    pub fn send_document(&self, document: impl Into<InputFileVariant>) -> SendDocument {
        SendDocument::new(self.id, document)
    }

    pub fn send_location(
        &self,
        latitude: f32,
        longitude: f32,
        horizontal_accuracy: f32,
    ) -> SendLocation {
        SendLocation::new(self.id, latitude, longitude, horizontal_accuracy)
    }

    pub fn send_media_group(&self) -> SendMediaGroup {
        SendMediaGroup::new(self.id)
    }

    pub fn send_message(&self, text: impl Into<String>) -> SendMessage {
        SendMessage::new(self.id, text)
    }

    pub fn send_photo(&self, photo: impl Into<InputFileVariant>) -> SendPhoto {
        SendPhoto::new(self.id, photo)
    }

    pub fn send_poll(&self, question: impl Into<String>, options: Vec<String>) -> SendPoll {
        SendPoll::new_regular(self.id, question, options)
    }

    pub fn send_quiz(
        &self,
        question: impl Into<String>,
        options: Vec<String>,
        correct_option_id: u32,
    ) -> SendPoll {
        SendPoll::new_quiz(self.id, question, options, correct_option_id)
    }

    pub fn send_venue(
        &self,
        latitude: f32,
        longitude: f32,
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> SendVenue {
        SendVenue::new(self.id, latitude, longitude, title, address)
    }

    pub fn send_video(&self, video: impl Into<InputFileVariant>) -> SendVideo {
        SendVideo::new(self.id, video)
    }

    pub fn send_video_note(&self, video_note: impl Into<InputFileVariant>) -> SendVideoNote {
        SendVideoNote::new(self.id, video_note)
    }

    pub fn send_voice(&self, voice: impl Into<InputFileVariant>) -> SendVoice {
        SendVoice::new(self.id, voice)
    }
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

impl ChatMember {
    pub fn user(&self) -> &User {
        match self {
            ChatMember::Owner { user, .. }
            | ChatMember::Administrator { user, .. }
            | ChatMember::Member { user }
            | ChatMember::Restricted { user, .. }
            | ChatMember::Left { user }
            | ChatMember::Banned { user, .. } => user,
        }
    }

    pub fn is_anonymous(&self) -> Option<bool> {
        match self {
            ChatMember::Owner { is_anonymous, .. }
            | ChatMember::Administrator { is_anonymous, .. } => Some(*is_anonymous),
            _ => None,
        }
    }

    pub fn custom_title(&self) -> Option<&str> {
        match self {
            Self::Owner { custom_title, .. } | Self::Administrator { custom_title, .. } => {
                custom_title.as_deref()
            }
            _ => None,
        }
    }

    pub fn can_be_edited(&self) -> Option<bool> {
        match self {
            Self::Administrator { can_be_edited, .. } => Some(*can_be_edited),
            _ => None,
        }
    }

    pub fn can_manage_chat(&self) -> Option<bool> {
        match self {
            Self::Administrator {
                can_manage_chat, ..
            } => Some(*can_manage_chat),
            _ => None,
        }
    }

    pub fn can_delete_messages(&self) -> Option<bool> {
        match self {
            Self::Administrator {
                can_delete_messages,
                ..
            } => Some(*can_delete_messages),
            _ => None,
        }
    }

    pub fn can_manage_voice_chats(&self) -> Option<bool> {
        match self {
            Self::Administrator {
                can_manage_voice_chats,
                ..
            } => Some(*can_manage_voice_chats),
            _ => None,
        }
    }

    pub fn can_restrict_members(&self) -> Option<bool> {
        match self {
            Self::Administrator {
                can_restrict_members,
                ..
            } => Some(*can_restrict_members),
            _ => None,
        }
    }

    pub fn can_promote_members(&self) -> Option<bool> {
        match self {
            Self::Administrator {
                can_promote_members,
                ..
            } => Some(*can_promote_members),
            _ => None,
        }
    }

    pub fn can_change_info(&self) -> Option<bool> {
        match self {
            Self::Administrator {
                can_change_info, ..
            }
            | Self::Restricted {
                can_change_info, ..
            } => Some(*can_change_info),
            _ => None,
        }
    }

    pub fn can_invite_users(&self) -> Option<bool> {
        match self {
            Self::Administrator {
                can_invite_users, ..
            }
            | Self::Restricted {
                can_invite_users, ..
            } => Some(*can_invite_users),
            _ => None,
        }
    }

    pub fn can_edit_messages(&self) -> Option<bool> {
        match self {
            Self::Administrator {
                can_edit_messages, ..
            } => *can_edit_messages,
            _ => None,
        }
    }

    pub fn can_pin_messages(&self) -> Option<bool> {
        match self {
            Self::Administrator {
                can_pin_messages, ..
            } => *can_pin_messages,
            Self::Restricted {
                can_pin_messages, ..
            } => Some(*can_pin_messages),
            _ => None,
        }
    }

    pub fn can_post_messages(&self) -> Option<bool> {
        match self {
            Self::Administrator {
                can_post_messages, ..
            } => *can_post_messages,
            _ => None,
        }
    }

    pub fn can_send_messages(&self) -> Option<bool> {
        match self {
            Self::Restricted {
                can_send_messages, ..
            } => Some(*can_send_messages),
            _ => None,
        }
    }

    pub fn can_send_media_messages(&self) -> Option<bool> {
        match self {
            Self::Restricted {
                can_send_media_messages,
                ..
            } => Some(*can_send_media_messages),
            _ => None,
        }
    }

    pub fn can_send_polls(&self) -> Option<bool> {
        match self {
            Self::Restricted { can_send_polls, .. } => Some(*can_send_polls),
            _ => None,
        }
    }

    pub fn can_send_other_messages(&self) -> Option<bool> {
        match self {
            Self::Restricted {
                can_send_other_messages,
                ..
            } => Some(*can_send_other_messages),
            _ => None,
        }
    }

    pub fn can_add_web_page_previews(&self) -> Option<bool> {
        match self {
            Self::Restricted {
                can_add_web_page_previews,
                ..
            } => Some(*can_add_web_page_previews),
            _ => None,
        }
    }

    pub fn is_member(&self) -> bool {
        match self {
            Self::Owner { .. } | Self::Administrator { .. } | Self::Member { .. } => true,
            Self::Restricted { is_member, .. } => *is_member,
            ChatMember::Left { .. } | ChatMember::Banned { .. } => false,
        }
    }

    pub fn banned_until(&self) -> Option<u64> {
        match self {
            Self::Banned { until_date, .. } => Some(*until_date),
            _ => None,
        }
    }

    pub fn restricted_until(&self) -> Option<u64> {
        match self {
            Self::Restricted { until_date, .. } => Some(*until_date),
            _ => None,
        }
    }
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
