//! Types, requests, and responses related to chats.

use serde::{Deserialize, Serialize};

use crate::file::{InputFile, InputFileVariant, InputMedia};
use crate::markup::InlineKeyboardMarkup;
use crate::message::{
    ChatActionKind, DeleteMessage, EditMessageCaption, EditMessageMedia, EditMessageReplyMarkup,
    EditMessageText, Location, Message, SendAnimation, SendAudio, SendChatAction, SendContact,
    SendDice, SendDocument, SendLocation, SendMediaGroup, SendMessage, SendPhoto, SendPoll,
    SendVenue, SendVideo, SendVideoNote, SendVoice, StopPoll,
};
use crate::user::User;
use crate::{JsonMethod, TelegramMethod};

/// A chat room including supergroup, channel, and private chat.
#[derive(Debug, Deserialize)]
pub struct Chat {
    /// Unique identifier for this chat.
    pub id: i64,
    /// Type of chat.
    #[serde(flatten)]
    pub kind: ChatKind,
    /// Title, for supergroups, channels and group chats
    pub title: Option<String>,
    /// Username, for private chats, supergroups and channels if availablWe
    pub username: Option<String>,
    /// First name of the other party in a private chat
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat
    pub last_name: Option<String>,
    /// Chat photo.
    /// Returned only in [`GetChat`].
    pub photo: Option<ChatPhoto>,
    /// Bio of the other party in a private chat.
    /// Returned only in [`GetChat`].
    pub bio: Option<String>,
    /// Description, for groups, supergroups and channel chats.
    /// Returned only in [`GetChat`].
    pub description: Option<String>,
    /// Primary invite link, for groups, supergroups and channel chats.
    /// Returned only in [`GetChat`].
    pub invite_link: Option<String>,
    /// The most recent pinned message (by sending date).
    /// Returned only in [`GetChat`].
    pub pinned_message: Option<Box<Message>>,
    /// Default chat member permissions, for groups and supergroups.
    /// Returned only in [`GetChat`].
    pub permissions: Option<ChatPermissions>,
    /// Default chat member permissions, for groups and supergroups.
    /// Returned only in [`GetChat`].
    pub slow_mode_delay: Option<i32>,
    /// The time after which all messages sent to the chat will be automatically deleted; in seconds.
    /// Returned only in [`GetChat`].
    pub message_auto_delete_time: Option<i32>,
    /// For supergroups, name of group sticker set.
    /// Returned only in [`GetChat`].
    pub sticker_set_name: Option<String>,
    /// `true` if the bot can change the group sticker set.
    /// Returned only in [`GetChat`].
    pub can_get_sticker_set: Option<bool>,
    /// Unique identifier for the linked chat,
    /// i.e. the discussion group identifier for a channel and vice versa;
    /// for supergroups and channel chats.
    /// Returned only in [`GetChat`].
    pub linked_chat_id: Option<i32>,
    /// For supergroups, the location to which the supergroup is connected.
    /// Returned only in [`GetChat`].
    pub location: Option<ChatLocation>,
}

impl Chat {
    /// Creates a [`SendAnimation`] request which will send an animation to this chat.
    pub fn send_animation(&self, animation: impl Into<InputFileVariant>) -> SendAnimation {
        SendAnimation::new(self.id, animation)
    }

    /// Creates a [`SendAudio`] request which will send an audio to this chat.
    pub fn send_audio(&self, audio: impl Into<InputFileVariant>) -> SendAudio {
        SendAudio::new(self.id, audio)
    }

    /// Creates a [`SendChatAction`] request which will send a chat action to this chat.
    pub fn send_chat_action(&self, action: ChatActionKind) -> SendChatAction {
        SendChatAction::new(self.id, action)
    }

    /// Creates a [`SendContact`] request with given phone number and first name
    /// which will send a contact to this chat.
    pub fn send_contact(
        &self,
        phone_number: impl Into<String>,
        first_name: impl Into<String>,
    ) -> SendContact {
        SendContact::new(self.id, phone_number, first_name)
    }

    /// Creates a [`SendDice`] request which will send a dice to this chat.
    pub fn send_dice(&self) -> SendDice {
        SendDice::new(self.id)
    }

    /// Creates a [`SendChatAction`] request which will send an chat action to this chat.
    pub fn send_document(&self, document: impl Into<InputFileVariant>) -> SendDocument {
        SendDocument::new(self.id, document)
    }

    /// Creates a [`SendLocation`] request with given latitude, longitude, and horizontal accuracy
    /// which will send a location to this chat.
    pub fn send_location(
        &self,
        latitude: f32,
        longitude: f32,
        horizontal_accuracy: f32,
    ) -> SendLocation {
        SendLocation::new(self.id, latitude, longitude, horizontal_accuracy)
    }

    /// Creates a [`SendMediaGroup`] request which will send a group of media to this chat.
    pub fn send_media_group(&self) -> SendMediaGroup {
        SendMediaGroup::new(self.id)
    }

    /// Creates a [`SendMessage`] request which will send a text message to this chat.
    pub fn send_message(&self, text: impl Into<String>) -> SendMessage {
        SendMessage::new(self.id, text)
    }

    /// Creates a [`SendPhoto`] request which will send a photo to this chat.
    pub fn send_photo(&self, photo: impl Into<InputFileVariant>) -> SendPhoto {
        SendPhoto::new(self.id, photo)
    }

    /// Creates a [`SendPoll`] request with given question and options which will send a poll to this chat.
    pub fn send_poll(&self, question: impl Into<String>, options: Vec<String>) -> SendPoll {
        SendPoll::new_regular(self.id, question, options)
    }

    /// Creates a [`SendPoll`] request with given question, options and correct option id
    /// which will send a quiz to this chat.
    pub fn send_quiz(
        &self,
        question: impl Into<String>,
        options: Vec<String>,
        correct_option_id: u32,
    ) -> SendPoll {
        SendPoll::new_quiz(self.id, question, options, correct_option_id)
    }

    /// Creates a [`SendVenue`] request with given latitude, longitude, title, and address
    /// which will send a live location to this chat.
    pub fn send_venue(
        &self,
        latitude: f32,
        longitude: f32,
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> SendVenue {
        SendVenue::new(self.id, latitude, longitude, title, address)
    }

    /// Creates a [`SendVideo`] request which will send a video to this chat.
    pub fn send_video(&self, video: impl Into<InputFileVariant>) -> SendVideo {
        SendVideo::new(self.id, video)
    }

    /// Creates a [`SendVideoNote`] request which will send
    /// a [video note](https://telegram.org/blog/video-messages-and-telescope) to this chat.
    pub fn send_video_note(&self, video_note: impl Into<InputFileVariant>) -> SendVideoNote {
        SendVideoNote::new(self.id, video_note)
    }

    /// Creates a [`SendVoice`] request which will send a voice message to this chat.
    pub fn send_voice(&self, voice: impl Into<InputFileVariant>) -> SendVoice {
        SendVoice::new(self.id, voice)
    }

    /// Creates a [`BanChatMember`] request which will ban a user from this chat.
    pub fn ban(&self, user_id: i64) -> BanChatMember {
        BanChatMember::new(self.id, user_id)
    }

    /// Creates a [`UnbanChatMember`] request which will unban a user from this chat.
    pub fn unban(&self, user_id: i64) -> UnbanChatMember {
        UnbanChatMember::new(self.id, user_id)
    }

    /// Creates a [`RestrictChatMember`] request which will restrict permissions of a user from this chat.
    pub fn restrict(&self, user_id: i64, permissions: ChatPermissions) -> RestrictChatMember {
        RestrictChatMember::new(self.id, user_id, permissions)
    }

    /// Creates a [`PromoteChatMember`] request which will promote a user to an administrator from this chat.
    pub fn promote(&self, user_id: i64) -> PromoteChatMember {
        PromoteChatMember::new(self.id, user_id)
    }

    /// Creates a [`SetChatAdministratorCustomTitle`] request which will set a custom title for
    /// an administrator from this chat.
    pub fn set_administrator_title(
        &self,
        user_id: i64,
        custom_title: impl Into<String>,
    ) -> SetChatAdministratorCustomTitle {
        SetChatAdministratorCustomTitle::new(self.id, user_id, custom_title)
    }

    /// Creates a [`SetChatPermissions`] request which will ban a user from this chat.
    pub fn set_permissions(&self, permissions: ChatPermissions) -> SetChatPermissions {
        SetChatPermissions::new(self.id, permissions)
    }

    /// Creates a [`ExportChatInviteLink`] request which will create a new primary invite link to this chat.
    /// 
    /// Previously generated primary invite link will be revoked.
    pub fn export_invite_link(&self) -> ExportChatInviteLink {
        ExportChatInviteLink::new(self.id)
    }

    /// Creates a [`CreateChatInviteLink`] request which will create a new additional invite link to this chat.
    pub fn create_invite_link(&self) -> CreateChatInviteLink {
        CreateChatInviteLink::new(self.id)
    }

    /// Creates a [`EditChatInviteLink`] request which will edit the given invite link of this chat.
    pub fn edit_invite_link(&self, invite_link: impl Into<String>) -> EditChatInviteLink {
        EditChatInviteLink::new(self.id, invite_link)
    }

    /// Creates a [`RevokeChatInviteLink`] request which will revoke the given invite link of this chat.
    /// 
    /// If the primary invite link is revoked, a new link will be automatically generated.
    pub fn revoke_invite_link(&self, invite_link: impl Into<String>) -> RevokeChatInviteLink {
        RevokeChatInviteLink::new(self.id, invite_link)
    }

    /// Creates a [`ApproveChatJoinRequest`] request which will approve the join request of the given user.
    pub fn approve_join(&self, user_id: i64) -> ApproveChatJoinRequest {
        ApproveChatJoinRequest::new(self.id, user_id)
    }

    /// Creates a [`DeclineChatJoinRequest`] request which will decline the join request of the given user.
    pub fn decline_join(&self, user_id: i64) -> DeclineChatJoinRequest {
        DeclineChatJoinRequest::new(self.id, user_id)
    }

    /// Creates a [`SetChatPhoto`] request which will set the photo of this chat.
    pub fn set_photo(&self, photo: InputFile) -> SetChatPhoto {
        SetChatPhoto::new(self.id, photo)
    }

    /// Creates a [`DeleteChatPhoto`] request which will delete the photo of this chat.
    pub fn delete_photo(&self) -> DeleteChatPhoto {
        DeleteChatPhoto::new(self.id)
    }

    /// Creates a [`SetChatTitle`] request which will set the title of this chat.
    pub fn set_title(&self, title: impl Into<String>) -> SetChatTitle {
        SetChatTitle::new(self.id, title)
    }

    /// Creates a [`SetChatDescription`] request which will set the description of this chat to the given string.
    pub fn set_description(&self, description: impl Into<String>) -> SetChatDescription {
        SetChatDescription::new(self.id, description)
    }

    /// Creates a [`SetChatDescription`] request which will remove the chat description.
    pub fn remove_description(&self) -> SetChatDescription {
        SetChatDescription::new_empty(self.id)
    }

    /// Creates a [`PinChatMessage`] request which will pin the given message to this chat.
    pub fn pin_message(&self, message_id: i64) -> PinChatMessage {
        PinChatMessage::new(self.id, message_id)
    }

    /// Creates a [`UnpinChatMessage`] request which will unpin the pinned message from this chat.
    pub fn unpin_message(&self, message_id: i64) -> UnpinChatMessage {
        UnpinChatMessage::new(self.id, message_id)
    }

    /// Creates a [`UnpinChatMessage`] request which will unpin the latest pinned message from this chat.
    pub fn unpin_latest_message(&self) -> UnpinChatMessage {
        UnpinChatMessage::new_recent(self.id)
    }

    /// Creates a [`UnpinAllChatMessage`] request which will unpin all pinned messages from this chat.
    pub fn unpin_all_messages(&self) -> UnpinAllChatMessages {
        UnpinAllChatMessages::new(self.id)
    }

    /// Creates a [`LeaveChat`] request which will make the bot leave from this chat.
    pub fn leave(&self) -> LeaveChat {
        LeaveChat::new(self.id)
    }

    /// Creates a [`GetChat`] request which will return a detailed information of this chat.
    pub fn get_details(&self) -> GetChat {
        GetChat::new(self.id)
    }

    /// Creates a [`GetChatAdministrators`] request which will return a list of administrators in this chat.
    pub fn get_administrators(&self) -> GetChatAdministrators {
        GetChatAdministrators::new(self.id)
    }

    /// Creates a [`GetChatMemberCount`] request which will return the number of members in this chat.
    pub fn get_member_count(&self) -> GetChatMemberCount {
        GetChatMemberCount::new(self.id)
    }

    /// Creates a [`GetChatMember`] request which will return the information obout the given member in this chat.
    pub fn get_member(&self, user_id: i64) -> GetChatMember {
        GetChatMember::new(self.id, user_id)
    }

    /// Creates a [`SetChatStickerSet`] request which will set this chat's sticker set.
    pub fn set_sticker_set(&self, sticker_set_name: impl Into<String>) -> SetChatStickerSet {
        SetChatStickerSet::new(self.id, sticker_set_name)
    }

    /// Creates a [`DeleteChatStickerSet`] request which will delete this chat's sticker set.
    pub fn delete_sticker_set(&self) -> DeleteChatStickerSet {
        DeleteChatStickerSet::new(self.id)
    }

    /// Creates an [`EditMessageText`] request which will change the text of given message in this chat.
    pub fn edit_text_of(&self, message_id: i64, text: impl Into<String>) -> EditMessageText {
        EditMessageText::new(self.id, message_id, text)
    }

    /// Creates an [`EditMessageCaption`] request which will remove the caption of given message in this chat.
    pub fn remove_caption_of(&self, message_id: i64) -> EditMessageCaption {
        EditMessageCaption::new_empty(self.id, message_id)
    }

    /// Creates an [`EditMessageCaption`] request which will change the caption of given message in this chat.
    pub fn edit_caption_of(
        &self,
        message_id: i64,
        caption: impl Into<String>,
    ) -> EditMessageCaption {
        EditMessageCaption::new(self.id, message_id, caption)
    }

    /// Creates an [`EditMessageMedia`] request which will change the media content of given message in this chat.
    pub fn edit_media_of(&self, message_id: i64, media: impl Into<InputMedia>) -> EditMessageMedia {
        EditMessageMedia::new(self.id, message_id, media)
    }

    /// Creates an [`EditMessageReplyMarkup`] request which will remove the reply markup of the given message in this chat.
    pub fn remove_reply_markup_of(&self, message_id: i64) -> EditMessageReplyMarkup {
        EditMessageReplyMarkup::new_empty(self.id, message_id)
    }

    /// Creates an [`EditMessageReplyMarkup`] request which will change the reply markup of the given message in this chat.
    pub fn edit_reply_markup_of(
        &self,
        message_id: i64,
        reply_markup: impl Into<InlineKeyboardMarkup>,
    ) -> EditMessageReplyMarkup {
        EditMessageReplyMarkup::new(self.id, message_id, reply_markup)
    }

    /// Creates a [`StopPoll`] request which will stop the poll with given message id in this chat.
    pub fn stop_poll(&self, message_id: i64) -> StopPoll {
        StopPoll::new(self.id, message_id)
    }

    /// Creates a [`DeleteMessage`] request which will delete the given message from this chat.
    pub fn delete_message(&self, message_id: i64) -> DeleteMessage {
        DeleteMessage::new(self.id, message_id)
    }
}

/// Kinds of chat.
#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ChatKind {
    Private,
    Group,
    Supergroup,
    Channel,
}

/// A chat photo.
#[derive(Debug, Deserialize)]
pub struct ChatPhoto {
    /// File identifier of small (160x160) chat photo.
    /// 
    /// This can be used only for photo download
    /// and only for as long as the photo is not changed.
    pub small_file_id: String,
    /// Unique file identifier of small (160x160) chat photo.
    /// 
    /// This is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub small_file_unique_id: String,
    /// File identifier of big (640x640) chat photo.
    /// 
    /// This can be used only for photo download
    /// and only for as long as the photo is not changed.
    pub big_file_id: String,
    /// Unique file identifier of big (640x640) chat photo.
    /// 
    /// This is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub big_file_unique_id: String,
}

/// Location of a chat, especially supergroup.
#[derive(Debug, Deserialize)]
pub struct ChatLocation {
    /// The location to which the supergroup is connected.
    /// 
    /// It can't be a live location.
    pub location: Location,
    /// Location address; 1-64 characters, as defined by the chat owner
    pub address: String,
}

/// Describes actions that a non-administrator user is allowed to take in a chat.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatPermissions {
    /// `true` if the user is allowed to send text messages, contacts, locations and venues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,
    /// `true`, if the user is allowed to send audios, documents,
    /// photos, videos, video notes and voice notes, implies [`ChatPermissions::can_send_messages`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_media_messages: Option<bool>,
    /// `true` if the user is allowed to send polls, implies [`ChatPermissions::can_send_messages`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,
    /// `true` if the user is allowed to send animations, games, stickers
    /// and use inline bots, implies [`ChatPermissions::can_send_media_messages`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,
    /// `true` if the user is allowed to add web page previews to their messages,
    /// implies [`ChatPermissions::can_send_media_messages`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,
    /// `true` if the user is allowed to change the chat title, photo and other settings.
    /// 
    /// Ignored in public supergroups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// `true` if the user is allowed to invite new users to the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// `true` if the user is allowed to pin messages.
    /// 
    /// Ignored in public supergroups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
}

impl ChatPermissions {
    /// Creates a new [`ChatPermissions`] object with no option set
    pub fn new() -> Self {
        Default::default()
    }

    /// Allows sending text messages, contacts, locations and venues.
    pub fn allow_send_messages(self) -> Self {
        Self {
            can_send_messages: Some(true),
            ..self
        }
    }

    /// Allows sending audios, documents,
    /// photos, videos, video notes and voice notes.
    pub fn allow_send_media_messages(self) -> Self {
        Self {
            can_send_media_messages: Some(true),
            ..self
        }
    }

    /// Allows sending polls, implies [`ChatPermissions::can_send_media_messages`].
    pub fn allow_send_polls(self) -> Self {
        Self {
            can_send_polls: Some(true),
            ..self
        }
    }

    /// Allows sending animations, games, and stickers and using inline bots,
    /// implies [`ChatPermissions::can_send_media_messages`].
    pub fn allow_send_other_messages(self) -> Self {
        Self {
            can_send_other_messages: Some(true),
            ..self
        }
    }

    /// Allows adding web page previews to messages,
    /// implies [`ChatPermissions::can_send_media_messages`].
    pub fn allow_add_web_page_previews(self) -> Self {
        Self {
            can_add_web_page_previews: Some(true),
            ..self
        }
    }

    /// Allows changing chat title, photo, and other settings, ignored in public supergroups.
    pub fn allow_change_info(self) -> Self {
        Self {
            can_change_info: Some(true),
            ..self
        }
    }

    /// Allows inviting new users to the chat.
    pub fn allow_invite_users(self) -> Self {
        Self {
            can_invite_users: Some(true),
            ..self
        }
    }
    
    /// Allows pinning messages, ignored in public supergroups.
    pub fn allow_pin_messages(self) -> Self {
        Self {
            can_pin_messages: Some(true),
            ..self
        }
    }
}

/// Detailed information of a chat member.
/// 
/// Can be obtained with [`GetChatMember`]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "status")]
pub enum ChatMember {
    /// The owner of the chat with all privileges.
    #[serde(rename = "creator")]
    Owner {
        /// Information about the user.
        user: User,
        /// `true` if the user's presence in the chat is hidden.
        is_anonymous: bool,
        /// Custom title for this user.
        custom_title: Option<String>,
    },
    /// An administrator of the chat with some additional privileges.
    Administrator {
        /// Information about the user.
        user: User,
        /// `true` if the bot is allowed to edit administrator privileges of that user.
        can_be_edited: bool,
        /// `true` if the user's presence in the chat is hidden.
        is_anonymous: bool,
        /// `true` if the administrator can "manage" the chat.
        /// 
        /// With this privilege, the administrator can:
        /// - access the chat event log, chat statistics, and message statistics in channels
        /// - see channel members and anonymous administrators in supergroups
        /// - ignore slow mode
        /// 
        /// Implied by any other administrator privilege.
        can_manage_chat: bool,
        /// `true` if the administrator can delete messages of other users.
        can_delete_messages: bool,
        /// `true` if the administrator can manage voice chats.
        can_manage_voice_chats: bool,
        /// `true` if the administrator can restrict, ban or unban chat members.
        can_restrict_members: bool,
        /// `true` if the administrator can promote members.
        /// 
        /// With this privilege, the administrator can:
        /// - add new administrators with a subset of their own privilege
        /// - demote administrators that they has promoted directly
        /// - demote administrators promoted by other administrators they has appointed
        can_promote_members: bool,
        /// `true` if the user is allowed to change the chat title, photo and other settings.
        can_change_info: bool,
        /// `true` if the user is allowed to invite new users to the chat.
        can_invite_users: bool,
        /// `true` if the administrator can post in the channel; channels only.
        can_post_messages: Option<bool>,
        /// `true` if the administrator can edit messages of other users and can pin messages; channels only.
        can_edit_messages: Option<bool>,
        /// `true` if the user is allowed to pin messages; groups and supergroups only.
        can_pin_messages: Option<bool>,
        /// Custom title for this user.
        custom_title: Option<String>,
    },
    /// A chat member without additional privileges or restrictions.
    Member {
        /// Information about the user.
        user: User,
    },
    /// A chat member under some restrictions. Supergroups only.
    Restricted {
        /// Information about the user.
        user: User,
        /// `true` if the user is a member of the chat at the moment of the request.
        is_member: bool,
        /// `true` if the user is allowed to change the chat title, photo and other settings.
        can_change_info: bool,
        /// `true` if the user is allowed to invite new users to the chat.
        can_invite_users: bool,
        /// `true` if the user is allowed to pin messages.
        can_pin_messages: bool,
        /// `true` if the user is allowed to send text messages, contacts, locations and venues.
        can_send_messages: bool,
        /// `true` if the user is allowed to send audios, documents, photos, videos, video notes and voice notes.
        can_send_media_messages: bool,
        /// `true` if the user is allowed to send polls.
        can_send_polls: bool,
        /// `true` if the user is allowed to send animations, games, stickers and use inline bots.
        can_send_other_messages: bool,
        /// `true` if the user is allowed to add web page previews to their messages.
        can_add_web_page_previews: bool,
        /// Date when restrictions will be lifted for this user; unix time.
        /// If 0, then the user is restricted forever.
        until_date: u64,
    },
    /// A chat member that isn't currently a member of the chat, but may join it themselves.
    Left {
        /// Information about the user.
        user: User,
    },
    /// A chat member that was banned in the chat and can't return to the chat or view chat messages.
    #[serde(rename = "kicked")]
    Banned {
        /// Information about the user.
        user: User,
        /// Date when restrictions will be lifted for this user; unix time.
        /// If 0, then the user is banned forever.
        until_date: u64,
    },
}

impl ChatMember {
    /// Gets information about the user.
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

    /// `true` if the user's presence in the chat is hidden.
    pub fn is_anonymous(&self) -> Option<bool> {
        match self {
            ChatMember::Owner { is_anonymous, .. }
            | ChatMember::Administrator { is_anonymous, .. } => Some(*is_anonymous),
            _ => None,
        }
    }

    /// Custom title for this user.
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
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
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

/// Identifier of the chat or username of the supergroup (in the format `@supergroupusername`)
#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum ChatId {
    /// Identifier of the chat
    Id(i64),
    /// Username of the supergroup (`@supergroupname`)
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
#[derive(Clone, Serialize)]
pub struct BanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Date when the user will be unbanned, unix time.
    /// If user is banned for more than 366 days or less than 30 seconds from the current time
    /// they are considered to be banned forever.
    /// Applied for supergroups and channels only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<u64>,
    /// Pass _True_ to delete all messages from the chat for the user that is being removed.
    /// If _False_, the user will be able to see messages in the group that were sent before the user was removed.
    /// Always _True_ for supergroups and channels.
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[derive(Clone, Serialize)]
pub struct UnbanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Do nothing if the user is not banned
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[derive(Clone, Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[derive(Clone, Serialize)]
pub struct PromoteChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Pass _True_, if the administrator's presence in the chat is hidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// Pass _True_, if the administrator can access the chat event log, chat statistics, message statistics in channels,
    /// see channel members, see anonymous administrators in supergroups and ignore slow mode.
    /// Implied by any other administrator privilege
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_chat: Option<bool>,
    /// Pass _True_, if the administrator can delete messages of other users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,
    /// Pass _True_, if the administrator can manage voice chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_voice_chats: Option<bool>,
    /// Pass _True_, if the administrator can restrict, ban or unban chat members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,
    /// Pass _True_, if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted,
    /// directly or indirectly (promoted by administrators that were appointed by him)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,
    /// Pass _True_, if the administrator can change chat title, photo and other settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// Pass _True_, if the administrator can invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// Pass _True_, if the administrator can create channel posts, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// Pass _True_, if the administrator can edit messages of other users and can pin messages, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// Pass _True_, if the administrator can pin messages, supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[derive(Clone, Serialize)]
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
#[derive(Clone, Serialize)]
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
#[derive(Clone, Serialize)]
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

/// Use this method to create an additional invite link for a chat.
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// The link can be revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api#revokechatinvitelink).
/// Returns the new invite link as [ChatInviteLink](https://core.telegram.org/bots/api#chatinvitelink) object.
#[derive(Clone, Serialize)]
pub struct CreateChatInviteLink {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`)
    pub chat_id: ChatId,
    /// Invite link name; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<u64>,
    /// Maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<u32>,
    /// _True_, if users joining the chat via the link need to be approved by chat administrators. If _True_, *member_limit* can't be specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

impl CreateChatInviteLink {
    /// Create a new createChatInviteLink request
    pub fn new(chat_id: impl Into<ChatId>) -> Self {
        Self {
            chat_id: chat_id.into(),
            name: None,
            expire_date: None,
            member_limit: None,
            creates_join_request: None,
        }
    }
    /// Set invite link name
    pub fn with_name(self, name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..self
        }
    }
    /// Set link expire date
    pub fn with_expire_date(self, expire_date: u64) -> Self {
        Self {
            expire_date: Some(expire_date),
            ..self
        }
    }
    /// Set link member limit
    pub fn with_member_limit(self, member_limit: u32) -> Self {
        Self {
            member_limit: Some(member_limit),
            ..self
        }
    }
    /// Set `creates_join_request` to `true`
    pub fn create_join_reqeuest(self) -> Self {
        Self {
            creates_join_request: Some(true),
            ..self
        }
    }
}

impl TelegramMethod for CreateChatInviteLink {
    type Response = ChatInviteLink;

    fn name() -> &'static str {
        "createChatInviteLink"
    }
}

impl JsonMethod for CreateChatInviteLink {}

/// Use this method to edit a non-primary invite link created by the bot.
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api#chatinvitelink) object.
#[derive(Clone, Serialize)]
pub struct EditChatInviteLink {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`)
    pub chat_id: ChatId,
    /// The invite link to edit
    pub invite_link: String,
    /// Invite link name; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<u64>,
    /// Maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<u32>,
    /// _True_, if users joining the chat via the link need to be approved by chat administrators. If _True_, *member_limit* can't be specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

impl EditChatInviteLink {
    /// Create a new editChatInviteLink request
    pub fn new(chat_id: impl Into<ChatId>, invite_link: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
            name: None,
            expire_date: None,
            member_limit: None,
            creates_join_request: None,
        }
    }
    /// Set invite link name
    pub fn with_name(self, name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..self
        }
    }
    /// Set link expire date
    pub fn with_expire_date(self, expire_date: u64) -> Self {
        Self {
            expire_date: Some(expire_date),
            ..self
        }
    }
    /// Set link member limit
    pub fn with_member_limit(self, member_limit: u32) -> Self {
        Self {
            member_limit: Some(member_limit),
            ..self
        }
    }
    /// Set `creates_join_request` to `true`
    pub fn create_join_reqeuest(self) -> Self {
        Self {
            creates_join_request: Some(true),
            ..self
        }
    }
}

/// Use this method to revoke an invite link created by the bot.
/// If the primary link is revoked, a new link is automatically generated.
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// Returns the revoked invite link as [ChatInviteLink](https://core.telegram.org/bots/api#chatinvitelink) object.
#[derive(Clone, Serialize)]
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
#[derive(Clone, Serialize)]
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
#[derive(Clone, Serialize)]
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
#[derive(Clone, Serialize)]
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
#[derive(Clone, Serialize)]
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
#[derive(Clone, Serialize)]
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
#[derive(Clone, Serialize)]
pub struct SetChatDescription {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`)
    pub chat_id: ChatId,
    /// New chat description, 0-255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[derive(Clone, Serialize)]
pub struct PinChatMessage {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Identifier of a message to pin
    pub message_id: i64,
    /// Pass True, if it is not necessary to send a notification to all chat members about the new pinned message.
    /// Notifications are always disabled in channels and private chats.
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[derive(Clone, Serialize)]
pub struct UnpinChatMessage {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Identifier of a message to unpin. If not specified, the most recent pinned message (by sending date) will be unpinned.
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[derive(Clone, Serialize)]
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
#[derive(Clone, Serialize)]
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
#[derive(Clone, Serialize)]
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
#[derive(Clone, Serialize)]
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
#[derive(Clone, Serialize)]
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
#[derive(Clone, Serialize)]
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
#[derive(Clone, Serialize)]
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
#[derive(Clone, Serialize)]
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
