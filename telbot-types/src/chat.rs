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
    pub can_set_sticker_set: Option<bool>,
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

    /// Creates a [`UnpinAllChatMessages`] request which will unpin all pinned messages from this chat.
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

    /// Returns `true` if the user's presence in the chat is hidden.
    ///
    /// Returns `None` if the user is not the owner or an administrator.
    pub fn is_anonymous(&self) -> Option<bool> {
        match self {
            ChatMember::Owner { is_anonymous, .. }
            | ChatMember::Administrator { is_anonymous, .. } => Some(*is_anonymous),
            _ => None,
        }
    }

    /// Returns custom title for this user.
    pub fn custom_title(&self) -> Option<&str> {
        match self {
            Self::Owner { custom_title, .. } | Self::Administrator { custom_title, .. } => {
                custom_title.as_deref()
            }
            _ => None,
        }
    }

    /// Returns `true` if the bot is allowed to edit administrator privileges of this user.
    ///
    /// Returns `None` if the user is not an administrator.
    pub fn can_be_edited(&self) -> Option<bool> {
        match self {
            Self::Administrator { can_be_edited, .. } => Some(*can_be_edited),
            _ => None,
        }
    }

    /// Returns `true` if the administrator can "manage" the chat.
    ///
    /// Returns `None` if the user is not an administrator.
    ///
    /// See also [`ChatMember::Administrator::can_manage_chat`].
    pub fn can_manage_chat(&self) -> Option<bool> {
        match self {
            Self::Administrator {
                can_manage_chat, ..
            } => Some(*can_manage_chat),
            _ => None,
        }
    }

    /// Returns `true` if the administrator can delete messages of other users.
    ///
    /// Returns `None` if the user is not an administrator.
    pub fn can_delete_messages(&self) -> Option<bool> {
        match self {
            Self::Administrator {
                can_delete_messages,
                ..
            } => Some(*can_delete_messages),
            _ => None,
        }
    }

    /// Returns `true` if the administrator can manage voice chats.
    ///
    /// Returns `None` if the user is not an administrator.
    pub fn can_manage_voice_chats(&self) -> Option<bool> {
        match self {
            Self::Administrator {
                can_manage_voice_chats,
                ..
            } => Some(*can_manage_voice_chats),
            _ => None,
        }
    }

    /// Returns `true` if the administrator can restrict, ban or unban chat members.
    ///
    /// Returns `None` if the user is not an administrator.
    pub fn can_restrict_members(&self) -> Option<bool> {
        match self {
            Self::Administrator {
                can_restrict_members,
                ..
            } => Some(*can_restrict_members),
            _ => None,
        }
    }

    /// Returns `true` if the administrator can promote members.
    ///
    /// Returns `None` if the user is not an administrator.
    ///
    /// See also [`ChatMember::Administrator::can_promote_members`].
    pub fn can_promote_members(&self) -> Option<bool> {
        match self {
            Self::Administrator {
                can_promote_members,
                ..
            } => Some(*can_promote_members),
            _ => None,
        }
    }

    /// Returns `true` if the user is allowed to change the chat title, photo and other settings.
    ///
    /// Returns `None` if the user is not an administrator or a restricted user.
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

    /// Returns `true` if the user is allowed to invite new users to the chat.
    ///
    /// Returns `None` if the user is not an administrator or a restricted user.
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

    /// Returns `true` if the administrator can edit messages of other users and can pin messages; channels only.
    ///
    /// Returns `None` if the user is not an administrator or the privilege is not explicitly set.
    pub fn can_edit_messages(&self) -> Option<bool> {
        match self {
            Self::Administrator {
                can_edit_messages, ..
            } => *can_edit_messages,
            _ => None,
        }
    }

    /// Returns `true` if the user is allowed to pin messages; groups and supergroups only.
    ///
    /// Returns `None` if the user is not an administrator or a restricted user,
    /// or the privilege is not explicitly set.
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

    /// Returns `true` if the administrator can post in the channel; channels only.
    ///
    /// Returns `None` if the user is not an administrator or the privilege is not explicitly set.
    pub fn can_post_messages(&self) -> Option<bool> {
        match self {
            Self::Administrator {
                can_post_messages, ..
            } => *can_post_messages,
            _ => None,
        }
    }

    /// Returns `true` if the user is allowed to send text messages, contacts, locations and venues.
    ///
    /// Returns `None` if the user is not restricted.
    pub fn can_send_messages(&self) -> Option<bool> {
        match self {
            Self::Restricted {
                can_send_messages, ..
            } => Some(*can_send_messages),
            _ => None,
        }
    }

    /// Returns `true` if the user is allowed to send audios, documents, photos, videos, video notes and voice notes.
    ///
    /// Returns `None` if the user is not restricted.
    pub fn can_send_media_messages(&self) -> Option<bool> {
        match self {
            Self::Restricted {
                can_send_media_messages,
                ..
            } => Some(*can_send_media_messages),
            _ => None,
        }
    }

    /// Returns `true` if the user is allowed to send polls.
    ///
    /// Returns `None `if the user is not restricted.
    pub fn can_send_polls(&self) -> Option<bool> {
        match self {
            Self::Restricted { can_send_polls, .. } => Some(*can_send_polls),
            _ => None,
        }
    }

    /// Returns `true` if the user is allowed to send animations, games, stickers and use inline bots.
    ///
    /// Returns `None` if the user is not restricted.
    pub fn can_send_other_messages(&self) -> Option<bool> {
        match self {
            Self::Restricted {
                can_send_other_messages,
                ..
            } => Some(*can_send_other_messages),
            _ => None,
        }
    }

    /// Returns `true` if the user is allowed to add web page previews to their messages.
    ///
    /// Returns `None` if the user is not restricted.
    pub fn can_add_web_page_previews(&self) -> Option<bool> {
        match self {
            Self::Restricted {
                can_add_web_page_previews,
                ..
            } => Some(*can_add_web_page_previews),
            _ => None,
        }
    }

    /// Returns `true` if the user is currently a member of the chat.
    pub fn is_member(&self) -> bool {
        match self {
            Self::Owner { .. } | Self::Administrator { .. } | Self::Member { .. } => true,
            Self::Restricted { is_member, .. } => *is_member,
            ChatMember::Left { .. } | ChatMember::Banned { .. } => false,
        }
    }

    /// Returns the date when ban will be lifted for this user in unix time.
    ///
    /// Returns `None` if the user is not banned.
    ///
    /// See also [`ChatMember::Banned::until_date`].
    pub fn banned_until(&self) -> Option<u64> {
        match self {
            Self::Banned { until_date, .. } => Some(*until_date),
            _ => None,
        }
    }

    /// Returns the date when restrictions will be lifted for this user in unix time.
    ///
    /// Returns `None` if the user is not restricted.
    ///
    /// See also [`ChatMember::Restricted::until_date`].
    pub fn restricted_until(&self) -> Option<u64> {
        match self {
            Self::Restricted { until_date, .. } => Some(*until_date),
            _ => None,
        }
    }
}

/// An invite link for a chat.
#[derive(Debug, Deserialize)]
pub struct ChatInviteLink {
    /// The invite link.
    ///
    /// If the link was created by another chat administrator,
    /// then the second part of the link will be replaced with “…”.
    pub invite_link: String,
    /// Creator of the link.
    pub creator: User,
    /// `true` if the link is primary.
    pub is_primary: bool,
    /// `true` if the link is revoked.
    pub is_revoked: bool,
    /// Point in time (Unix timestamp) when the link will expire or has been expired.
    pub expire_date: Option<u64>,
    /// Maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999.
    pub member_limit: Option<u32>,
}

/// Changes in the status of a chat member.
#[derive(Debug, Deserialize)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to.
    pub chat: Chat,
    /// Performer of the action, which resulted in the change.
    pub from: User,
    /// Date the change was done in Unix time.
    pub date: u64,
    /// Previous information about the chat member.
    pub old_chat_member: ChatMember,
    /// New information about the chat member.
    pub new_chat_member: ChatMember,
    /// Chat invite link, which was used by the user to join the chat;
    /// for joining by invite link events only.
    pub invite_link: Option<ChatInviteLink>,
}

/// Identifier of the chat or username of the supergroup (in the format `@supergroupusername`)
///
/// You can pass values of type `i64`, `&str`, and `String` to parameters of type `impl Into<ChatId>`.
///
/// ```
/// # use telbot_types::chat::SetChatTitle;
/// let set_chat_title = SetChatTitle::new(123, "title");
/// let set_chat_title = SetChatTitle::new("@abcde", "title");
/// ```
#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum ChatId {
    /// Identifier of the chat.
    Id(i64),
    /// Username of the supergroup (`@supergroupname`).
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

/// Bans a user in a group, a supergroup or a channel.
///
/// In the case of supergroups and channels, the user will not be able to return to the chat
/// on their own using invite links, etc., unless unbanned first.
///
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
///
/// Returns `true` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#banchatmember)
#[derive(Clone, Serialize)]
pub struct BanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`).
    pub chat_id: ChatId,
    /// Unique identifier of the target user.
    pub user_id: i64,
    /// Date when the user will be unbanned, unix time.
    ///
    /// If user is banned for more than 366 days or less than 30 seconds from the current time
    /// they are considered to be banned forever.
    /// Applied for supergroups and channels only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<u64>,
    /// Set `true` to delete all messages from the chat for the user that is being removed.
    ///
    /// If `false`, the user will be able to see messages in the group that were sent before the user was removed.
    /// Always `true` for supergroups and channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}

impl BanChatMember {
    /// Creates a new [`BanChatMember`] request which will ban the user from the chat.
    pub fn new(chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
            until_date: None,
            revoke_messages: None,
        }
    }

    /// Sets the date at which the user will be unbanned in unix time.
    ///
    /// See also [`BanChatMember::until_date`].
    pub fn until_date(self, date: u64) -> Self {
        Self {
            until_date: Some(date),
            ..self
        }
    }

    /// Deletes all messages from the chat for the user that is being removed.
    ///
    /// See also [`BanChatMember::revoke_messages`].
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

/// Unbans a previously banned user in a supergroup or channel.
///
/// The user will **not** return to the group or channel automatically, but will be able to join via link, etc.
///
/// The bot must be an administrator for this to work.
///
/// By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it.
/// So if the user is a member of the chat they will also be **removed** from the chat.
/// If you don't want this, use the parameter [`UnbanChatMember::only_if_banned`] or [`UnbanChatMember::only_if_banned()`].
///
/// Returns `true` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#unbanchatmember)
#[derive(Clone, Serialize)]
pub struct UnbanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`).
    pub chat_id: ChatId,
    /// Unique identifier of the target user.
    pub user_id: i64,
    /// If `true`, do nothing if the user is not banned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_if_banned: Option<bool>,
}

impl UnbanChatMember {
    /// Creates a new [`UnbanChatMember`] request which will unban the user from the chat.
    pub fn new(chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
            only_if_banned: None,
        }
    }

    /// Do nothing if the user is not banned.
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

/// Restricts a user in a supergroup.
///
/// The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights.
///
/// Pass `true` for all permissions to lift restrictions from a user.
///
/// Returns `true` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#restrictchatmember)
#[derive(Clone, Serialize)]
pub struct RestrictChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`).
    pub chat_id: ChatId,
    /// Unique identifier of the target user.
    pub user_id: i64,
    /// A JSON-serialized object for new user permissions.
    pub permissions: ChatPermissions,
    /// Date when restrictions will be lifted for the user, unix time.
    ///
    /// If user is restricted for more than 366 days or less than 30 seconds from the current time,
    /// they are considered to be restricted forever.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<u64>,
}

impl RestrictChatMember {
    /// Creates a new [`RestrictChatMember`] request which will restrict the user permissions in the chat.
    pub fn new(chat_id: impl Into<ChatId>, user_id: i64, permissions: ChatPermissions) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
            permissions,
            until_date: None,
        }
    }

    pub fn new_lift(chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self::new(
            chat_id,
            user_id,
            ChatPermissions {
                can_send_messages: Some(true),
                can_send_media_messages: Some(true),
                can_send_polls: Some(true),
                can_send_other_messages: Some(true),
                can_add_web_page_previews: Some(true),
                can_change_info: Some(true),
                can_invite_users: Some(true),
                can_pin_messages: Some(true),
            },
        )
    }

    /// Sets the date at which the restriction wil be lifted.
    ///
    /// See also [`RestrictChatMember::until_date`].
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

/// Promotes or demotes a user in a supergroup or a channel.
///
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
///
/// Pass `false` for all boolean parameters to demote a user.
///
/// Returns `true` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#promotechatmember)
#[derive(Clone, Serialize)]
pub struct PromoteChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`).
    pub chat_id: ChatId,
    /// Unique identifier of the target user.
    pub user_id: i64,
    /// Set `true` if the administrator's presence in the chat is hidden.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// Set `true` if the administrator can "manage" the chat.
    ///
    /// See also [`ChatMember::Administrator::can_manage_chat`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_chat: Option<bool>,
    /// Set `true` if the administrator can delete messages of other users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,
    /// Set `true` if the administrator can manage voice chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_voice_chats: Option<bool>,
    /// Set `true` if the administrator can restrict, ban or unban chat members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,
    /// Set `true` if the administrator can promote members.
    ///
    /// See also [`ChatMember::Administrator::can_promote_members`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,
    /// Set `true` if the administrator can change chat title, photo and other settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// Set `true` if the administrator can invite new users to the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// Set `true` if the administrator can create channel posts, channels only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// Set `true` if the administrator can edit messages of other users and can pin messages, channels only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// Set `true` if the administrator can pin messages, supergroups only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
}

impl PromoteChatMember {
    /// Creates a new [`PromoteChatMember`] request which will promote the user in the chat.
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

    /// Creates a new [`PromoteChatMember`] request which will demote the user in the chat.
    ///
    /// It creates a new [`PromoteChatMember`] request with all options disabled.
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

    /// Sets if the user's presence in the chat is hidden.
    pub fn with_anonymous(self, is_anonymous: bool) -> Self {
        Self {
            is_anonymous: Some(is_anonymous),
            ..self
        }
    }

    /// Sets if the user can "manage" the chat.
    ///
    /// See also [`ChatMember::Administrator::can_manage_chat`].
    pub fn with_manage_chat(self, can_manage_chat: bool) -> Self {
        Self {
            can_manage_chat: Some(can_manage_chat),
            ..self
        }
    }

    /// Sets if the user can delete messages of other users.
    pub fn with_delete_messages(self, can_delete_messages: bool) -> Self {
        Self {
            can_delete_messages: Some(can_delete_messages),
            ..self
        }
    }

    /// Sets if the user can manage voice chats.
    pub fn with_manage_voice_chats(self, can_manage_voice_chats: bool) -> Self {
        Self {
            can_manage_voice_chats: Some(can_manage_voice_chats),
            ..self
        }
    }

    /// Sets if the user can restrict, ban or unban chat members.
    pub fn with_restrict_members(self, can_restrict_members: bool) -> Self {
        Self {
            can_restrict_members: Some(can_restrict_members),
            ..self
        }
    }

    /// Sets if the user can promote members.
    ///
    /// See also [`ChatMember::Administrator::can_promote_members`].
    pub fn with_promote_members(self, can_promote_members: bool) -> Self {
        Self {
            can_promote_members: Some(can_promote_members),
            ..self
        }
    }

    /// Sets if the user can change the chat title, photo and other settings.
    pub fn with_change_info(self, can_change_info: bool) -> Self {
        Self {
            can_change_info: Some(can_change_info),
            ..self
        }
    }

    /// Sets if the user can invite new users to the chat.
    pub fn with_invite_users(self, can_invite_users: bool) -> Self {
        Self {
            can_invite_users: Some(can_invite_users),
            ..self
        }
    }

    /// Sets if the user can post in the channel; channels only.
    pub fn with_post_messages(self, can_post_messages: bool) -> Self {
        Self {
            can_post_messages: Some(can_post_messages),
            ..self
        }
    }

    /// Sets if the user can edit messages of other users and can pin messages; channels only.
    pub fn with_edit_messages(self, can_edit_messages: bool) -> Self {
        Self {
            can_edit_messages: Some(can_edit_messages),
            ..self
        }
    }

    /// Sets if the user can pin messages; groups and supergroups only.
    pub fn with_pin_messages(self, can_pin_messages: bool) -> Self {
        Self {
            can_pin_messages: Some(can_pin_messages),
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

/// Sets a custom title for an administrator in a supergroup promoted by the bot.
///
/// Returns `true` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#setchatadministratorcustomtitle)
#[derive(Clone, Serialize)]
pub struct SetChatAdministratorCustomTitle {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`).
    pub chat_id: ChatId,
    /// Unique identifier of the target user.
    pub user_id: i64,
    /// New custom title for the administrator; 0-16 characters, emoji are not allowed.
    pub custom_title: String,
}

impl SetChatAdministratorCustomTitle {
    /// Creates a new [`SetChatAdministratorCustomTitle`] request which will set the user's title in the chat.
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

/// Sets default chat permissions for all members.
///
/// The bot must be an administrator in the group or a supergroup for this to work
/// and must have the [`ChatMember::Administrator::can_restrict_members`] administrator rights.
///
/// Returns `true` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#setchatpermissions)
#[derive(Clone, Serialize)]
pub struct SetChatPermissions {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`).
    pub chat_id: ChatId,
    /// New user permissions.
    pub permissions: ChatPermissions,
}

impl SetChatPermissions {
    /// Creates a new [`SetChatPermissions`] request which will set default chat permissions in the chat.
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

/// Generates a new primary invite link for a chat;
/// any previously generated primary link is revoked.
///
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// Returns the new invite link as `String` on success.
///
/// Note: Each administrator in a chat generates their own invite links.
/// Bots can't use invite links generated by other administrators.
/// If you want your bot to work with invite links,
/// it will need to generate its own link using [`ExportChatInviteLink`]
/// or by calling the [`GetChat`] method.
/// If your bot needs to generate a new primary invite link replacing its previous one,
/// use [`ExportChatInviteLink`] again.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#exportchatinvitelink)
#[derive(Clone, Serialize)]
pub struct ExportChatInviteLink {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`).
    pub chat_id: ChatId,
}

impl ExportChatInviteLink {
    /// Creates a new [`ExportChatInviteLink`] request which will generate a new primary invite link for the chat.
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

/// Creates an additional invite link for a chat.
///
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
///
/// The link can be revoked using the method [`RevokeChatInviteLink`].
///
/// Returns the new invite link as [`ChatInviteLink`] object.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#createchatinvitelink)
#[derive(Clone, Serialize)]
pub struct CreateChatInviteLink {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`).
    pub chat_id: ChatId,
    /// Invite link name; 0-32 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<u64>,
    /// Maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<u32>,
    /// `true` if users joining the chat via the link need to be approved by chat administrators.
    ///
    /// If `true`, the member limit can't be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

impl CreateChatInviteLink {
    /// Creates a new [`CreateChatInviteLink`] request which will create a new additional invite link for the chat.
    pub fn new(chat_id: impl Into<ChatId>) -> Self {
        Self {
            chat_id: chat_id.into(),
            name: None,
            expire_date: None,
            member_limit: None,
            creates_join_request: None,
        }
    }

    /// Sets the name of invite link.
    pub fn with_name(self, name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..self
        }
    }

    /// Sets the time at which the invite link is expired in unix time.
    pub fn with_expire_date(self, expire_date: u64) -> Self {
        Self {
            expire_date: Some(expire_date),
            ..self
        }
    }

    /// Sets the maximum number of users the chat can have after the user joins the chat.
    ///
    /// If set, join request cannot be created.
    pub fn with_member_limit(self, member_limit: u32) -> Self {
        Self {
            member_limit: Some(member_limit),
            ..self
        }
    }

    /// Makes users joining the chat via the link need to be approved by chat administrators.
    ///
    /// If used, member limit cannot be set.
    pub fn create_join_request(self) -> Self {
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

/// Edits a non-primary invite link created by the bot.
///
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
///
/// Returns the edited invite link as a [`ChatInviteLink`] object.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#editchatinvitelink)
#[derive(Clone, Serialize)]
pub struct EditChatInviteLink {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`).
    pub chat_id: ChatId,
    /// The invite link to edit.
    pub invite_link: String,
    /// Invite link name; 0-32 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<u64>,
    /// Maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<u32>,
    /// `true` if users joining the chat via the link need to be approved by chat administrators.
    ///
    /// If `true`, the member limit can't be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

impl EditChatInviteLink {
    /// Creates a new [`EditChatInviteLink`] request which will edit an existing invite link for the chat.
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
    /// Sets the name of invite link.
    pub fn with_name(self, name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..self
        }
    }
    /// Sets the time at which the invite link is expired in unix time.
    pub fn with_expire_date(self, expire_date: u64) -> Self {
        Self {
            expire_date: Some(expire_date),
            ..self
        }
    }
    /// Sets the maximum number of users the chat can have after the user joins the chat.
    ///
    /// If set, join request cannot be created.
    pub fn with_member_limit(self, member_limit: u32) -> Self {
        Self {
            member_limit: Some(member_limit),
            ..self
        }
    }
    /// Makes users joining the chat via the link need to be approved by chat administrators.
    ///
    /// If used, member limit cannot be set.
    pub fn create_join_reqeuest(self) -> Self {
        Self {
            creates_join_request: Some(true),
            ..self
        }
    }
}

/// Revokes an invite link created by the bot.
///
/// If the primary link is revoked, a new link is automatically generated.
///
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
///
/// Returns the revoked invite link as [`ChatInviteLink`] object.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#revokechatinvitelink)
#[derive(Clone, Serialize)]
pub struct RevokeChatInviteLink {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`).
    pub chat_id: ChatId,
    /// The invite link to revoke.
    pub invite_link: String,
}

impl RevokeChatInviteLink {
    /// Creates a new [`RevokeChatInviteLink`] request which will revoke an existing invite link for the chat.
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

/// Approves a chat join request.
///
/// The bot must be an administrator in the chat for this to work and must have the [`ChatMember::Administrator::can_invite_users`] administrator right.
///
/// Returns `true` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#approvechatjoinrequest)
#[derive(Clone, Serialize)]
pub struct ApproveChatJoinRequest {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format `@username`).
    pub chat_id: ChatId,
    /// Unique identifier of the target user.
    pub user_id: i64,
}

impl ApproveChatJoinRequest {
    /// Creates a new [`ApproveChatJoinRequest`] request which will approve a chat join request to the chat of the target user.
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

/// Declines a chat join request.
///
/// The bot must be an administrator in the chat for this to work and must have the [`ChatMember::Administrator::can_invite_users`] administrator right.
///
/// Returns `true` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#declinechatjoinrequest)
#[derive(Clone, Serialize)]
pub struct DeclineChatJoinRequest {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`).
    pub chat_id: ChatId,
    /// Unique identifier of the target user.
    pub user_id: i64,
}

impl DeclineChatJoinRequest {
    /// Creates a new [`DeclineChatJoinRequest`] request which will decline a chat join request to the chat of the target user.
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

/// Sets a new profile photo for the chat.
///
/// Photos can't be changed for private chats.
///
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
///
/// Returns `true` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#setchatphoto)
#[derive(Clone, Serialize)]
pub struct SetChatPhoto {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`).
    pub chat_id: ChatId,
    /// New chat photo, uploaded using multipart/form-data.
    pub photo: InputFile,
}

impl SetChatPhoto {
    /// Creates a new [`SetChatPhoto`] request which will set a new profile photo for the chat.
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

/// Deletes a chat photo.
///
/// Photos can't be changed for private chats.
///
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
///
/// Returns `true` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#deletechatphoto)
#[derive(Clone, Serialize)]
pub struct DeleteChatPhoto {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`).
    pub chat_id: ChatId,
}

impl DeleteChatPhoto {
    /// Creates a new [`DeleteChatPhoto`] request which will delete the profile photo of the chat.
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

/// Changes the title of a chat.
///
/// Titles can't be changed for private chats.
///
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
///
/// Returns `true` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#setchattitle)
#[derive(Clone, Serialize)]
pub struct SetChatTitle {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`).
    pub chat_id: ChatId,
    /// New chat title, 1-255 characters.
    pub title: String,
}

impl SetChatTitle {
    /// Creates a new [`SetChatTitle`] request which will change the title of the chat.
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

/// Changes the description of a group, a supergroup or a channel.
///
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
///
/// Returns `true` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#setchatdescription)
#[derive(Clone, Serialize)]
pub struct SetChatDescription {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@username`).
    pub chat_id: ChatId,
    /// New chat description, 0-255 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl SetChatDescription {
    /// Creates a new [`SetChatDescription`] request which will empty the description of the chat.
    pub fn new_empty(chat_id: impl Into<ChatId>) -> Self {
        Self {
            chat_id: chat_id.into(),
            description: None,
        }
    }

    /// Creates a new [`SetChatDescription`] request which will set the description of the chat.
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

/// Adds a message to the list of pinned messages in a chat.
///
/// If the chat is not a private chat, the bot must be an administrator in the chat for this to work
/// and must have the [`ChatMember::Administrator::can_pin_messages`] administrator right in a supergroup or [`ChatMember::Administrator::can_edit_messages`] administrator right in a channel.
///
/// Returns `true` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#pinchatmessage)
#[derive(Clone, Serialize)]
pub struct PinChatMessage {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`).
    pub chat_id: ChatId,
    /// Identifier of a message to pin.
    pub message_id: i64,
    /// Pass `true`, if it is not necessary to send a notification to all chat members about the new pinned message.
    /// Notifications are always disabled in channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
}

impl PinChatMessage {
    /// Creates a new [`PinChatMessage`] request which will pin a message in the chat.
    pub fn new(chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id,
            disable_notification: None,
        }
    }

    /// Disables notification for the pinned message.
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

/// Removes a message from the list of pinned messages in a chat.
///
/// If the chat is not a private chat, the bot must be an administrator in the chat for this to work
/// and must have the [`ChatMember::Administrator::can_pin_messages`] administrator right in a supergroup or [`ChatMember::Administrator::can_edit_messages`] administrator right in a channel.
///
/// Returns `true` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#unpinchatmessage)
#[derive(Clone, Serialize)]
pub struct UnpinChatMessage {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`).
    pub chat_id: ChatId,
    /// Identifier of a message to unpin.
    ///
    /// If not specified, the most recent pinned message (by sending date) will be unpinned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
}

impl UnpinChatMessage {
    /// Creates a new [`UnpinChatMessage`] request which will unpin the most recent pinned message in the chat.
    pub fn new_recent(chat_id: impl Into<ChatId>) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id: None,
        }
    }

    /// Creates a new [`UnpinChatMessage`] request which will unpin the specified message in the chat.
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

/// Clears the list of pinned messages in a chat.
///
/// If the chat is not a private chat, the bot must be an administrator in the chat for this to work
/// and must have the [`ChatMember::Administrator::can_pin_messages`] administrator right in a supergroup or [`ChatMember::Administrator::can_edit_messages`] administrator right in a channel.
///
/// Returns `true` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#unpinallchatmessages)
#[derive(Clone, Serialize)]
pub struct UnpinAllChatMessages {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`).
    pub chat_id: ChatId,
}

impl UnpinAllChatMessages {
    /// Creates a new [`UnpinAllChatMessages`] request which will unpin all messages in the chat.
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

/// Leaves a group, supergroup or channel.
///
/// Returns `true` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#leavechat)
#[derive(Clone, Serialize)]
pub struct LeaveChat {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`).
    pub chat_id: ChatId,
}

impl LeaveChat {
    /// Creates a new [`LeaveChat`] request which will make the bot leave the chat.
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

/// Gets up to date information about the chat
/// (current name of the user for one-on-one conversations, current username of a user, group or channel, etc.).
///
/// Returns a [`Chat`] object on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#getchat)
#[derive(Clone, Serialize)]
pub struct GetChat {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`).
    pub chat_id: ChatId,
}

impl GetChat {
    /// Creates a new [`GetChat`] request which will get information about the chat.
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

/// Gets a list of administrators in a chat.
///
/// On success, returns an Array of [`ChatMember`] objects
/// that contains information about all chat administrators except other bots.
/// If the chat is a group or a supergroup and no administrators were appointed, only the creator will be returned.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#getchatadministrators)
#[derive(Clone, Serialize)]
pub struct GetChatAdministrators {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`).
    pub chat_id: ChatId,
}

impl GetChatAdministrators {
    /// Creates a new [`GetChatAdministrators`] request which will get a list of administrators in the chat.
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

/// Gets the number of members in a chat.
///
/// Returns `u32` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#getchatmembercount)
#[derive(Clone, Serialize)]
pub struct GetChatMemberCount {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`).
    pub chat_id: ChatId,
}

impl GetChatMemberCount {
    /// Creates a new [`GetChatMemberCount`] request which will get the number of members in the chat.
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

/// Gets information about a member of a chat.
///
/// Returns a [`ChatMember`] object on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#getchatmember)
#[derive(Clone, Serialize)]
pub struct GetChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`).
    pub chat_id: ChatId,
    /// Unique identifier of the target user.
    pub user_id: i64,
}

impl GetChatMember {
    /// Creates a new [`GetChatMember`] request which will get information about a member of the chat.
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

/// Sets a new group sticker set for a supergroup.
///
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// Use the field [`Chat::can_set_sticker_set`] optionally returned in [`GetChat`] requests to check if the bot can use this method.
///
/// Returns `true` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#setchatstickerset)
#[derive(Clone, Serialize)]
pub struct SetChatStickerSet {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`).
    pub chat_id: ChatId,
    /// Name of the sticker set to be set as the group sticker set.
    pub sticker_set_name: String,
}

impl SetChatStickerSet {
    /// Creates a new [`SetChatStickerSet`] request which will set a new group sticker set for a supergroup.
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

/// Deletes a group sticker set from a supergroup.
///
/// The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// Use the field [`Chat::can_set_sticker_set`] optionally returned in [`GetChat`] requests to check if the bot can use this method.
///
/// Returns `true` on success.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#deletechatstickerset)
#[derive(Clone, Serialize)]
pub struct DeleteChatStickerSet {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`).
    pub chat_id: ChatId,
}

impl DeleteChatStickerSet {
    /// Creates a new [`DeleteChatStickerSet`] request which will delete a group sticker set from a supergroup.
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
