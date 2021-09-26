use std::collections::HashMap;

use crate::chat::ChatId;
use crate::file::{InputFile, InputFileVariant, InputMedia};
use crate::markup::{MessageEntity, ParseMode, ReplyMarkup};
use crate::message::Message;
use crate::user::User;
use crate::{FileMethod, JsonMethod, TelegramMethod};
use serde::Serialize;

/// This object represents a bot command.
pub struct BotCommand {
    /// Text of the command, 1-32 characters.
    /// Can contain only lowercase English letters, digits and underscores.
    pub command: String,
    /// Description of the command, 3-256 characters.
    pub description: String,
}

/// This object represents the scope to which bot commands are applied.
///
/// Currently, the following 7 scopes are supported:
/// - BotCommandScopeDefault
/// - BotCommandScopeAllPrivateChats
/// - BotCommandScopeAllGroupChats
/// - BotCommandScopeAllChatAdministrators
/// - BotCommandScopeChat
/// - BotCommandScopeChatAdministrators
/// - BotCommandScopeChatMember
///
/// # Determining list of commands
///
/// The following algorithm is used to determine the list of commands for a particular user viewing the bot menu. The first list of commands which is set is returned:
///
/// ## Commands in the chat with the bot
///
/// - botCommandScopeChat + language_code
/// - botCommandScopeChat
/// - botCommandScopeAllPrivateChats + language_code
/// - botCommandScopeAllPrivateChats
/// - botCommandScopeDefault + language_code
/// - botCommandScopeDefault
///
/// ## Commands in group and supergroup chats
///
/// - botCommandScopeChatMember + language_code
/// - botCommandScopeChatMember
/// - botCommandScopeChatAdministrators + language_code (admins only)
/// - botCommandScopeChatAdministrators (admins only)
/// - botCommandScopeChat + language_code
/// - botCommandScopeChat
/// - botCommandScopeAllChatAdministrators + language_code (admins only)
/// - botCommandScopeAllChatAdministrators (admins only)
/// - botCommandScopeAllGroupChats + language_code
/// - botCommandScopeAllGroupChats
/// - botCommandScopeDefault + language_code
/// - botCommandScopeDefault
pub enum BotCommandScope {
    /// Default commands are used if no commands with a narrower scope are specified for the user.
    Default,
    /// Covers all private chats.
    AllPrivateChats,
    /// Covers all group and supergroup chats.
    AllGroupChats,
    /// Cvoers all group and supergroup chat administrators.
    AllChatAdministrators,
    /// Covers a specific chat.
    Chat {
        /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
        chat_id: ChatId,
    },
    /// Covers all administrators of a specific group or supergroup chat.
    ChatAdministrators {
        /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
        chat_id: ChatId,
    },
    /// Covers a specific member of a group or supergroup chat.
    ChatMember {
        /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
        chat_id: ChatId,
        /// Unique identifier of the target user
        user_id: i64,
    },
}

/// A simple method for testing your bot's auth token. Requires no parameters.
///
/// Returns basic information about the bot in form of a User object.
#[derive(Serialize)]
pub struct GetMe;

impl TelegramMethod for GetMe {
    type Response = User;

    fn name() -> &'static str {
        "getMe"
    }
}

impl JsonMethod for GetMe {}

/// Use this method to log out from the cloud Bot API server before launching the bot locally.
///
/// You **must** log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates.
/// After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes.
/// Returns *True* on success. Requires no parameters.
#[derive(Serialize)]
pub struct LogOut;

impl TelegramMethod for LogOut {
    type Response = bool;

    fn name() -> &'static str {
        "logOut"
    }
}

impl JsonMethod for LogOut {}

/// Use this method to close the bot instance before moving it from one local server to another.
/// You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart.
/// The method will return error 429 in the first 10 minutes after the bot is launched.
/// Returns True on success. Requires no parameters.
#[derive(Serialize)]
pub struct Close;

impl TelegramMethod for Close {
    type Response = bool;

    fn name() -> &'static str {
        "close"
    }
}

impl JsonMethod for Close {}

/// Use this method to send text messages. On success, the sent Message is returned.
#[derive(Serialize)]
pub struct SendMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Text of the message to be sent, 1-4096 characters after entities parsing
    pub text: String,
    /// Mode for parsing entities in the message text.
    /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// List of special entities that appear in message text,
    /// which can be specified instead of *parse_mode*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Disables link previews for links in the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages).
    /// Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass *True*, if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options.
    /// A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating),
    /// [custom reply keyboard](https://core.telegram.org/bots#keyboards),
    /// instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendMessage {
    /// Create a new sendMessage request
    pub fn new(chat_id: impl Into<ChatId>, text: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            text: text.into(),
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    /// Set parse mode
    pub fn with_parse_mode(self, parse_mode: ParseMode) -> Self {
        Self {
            parse_mode: Some(parse_mode),
            ..self
        }
    }
    /// Set entities
    pub fn with_entities(self, entities: Vec<MessageEntity>) -> Self {
        Self {
            entities: Some(entities),
            ..self
        }
    }
    /// Add one entity
    pub fn with_entity(mut self, entity: MessageEntity) -> Self {
        let entities = self.entities.get_or_insert_with(Default::default);
        entities.push(entity);
        self
    }
    /// Disable web preview
    pub fn disable_web_page_preview(self) -> Self {
        Self {
            disable_web_page_preview: Some(true),
            ..self
        }
    }
    /// Disable notification
    pub fn disable_notification(self) -> Self {
        Self {
            disable_notification: Some(true),
            ..self
        }
    }
    /// Reply to message
    pub fn reply_to(self, message_id: i64) -> Self {
        Self {
            reply_to_message_id: Some(message_id),
            ..self
        }
    }
    /// Allow sending message even if the replying message isn't present
    pub fn allow_sending_without_reply(self) -> Self {
        Self {
            allow_sending_without_reply: Some(true),
            ..self
        }
    }
    /// Set reply markup
    pub fn with_reply_markup(self, markup: impl Into<ReplyMarkup>) -> Self {
        Self {
            reply_markup: Some(markup.into()),
            ..self
        }
    }
}

impl TelegramMethod for SendMessage {
    type Response = Message;

    fn name() -> &'static str {
        "sendMessage"
    }
}

impl JsonMethod for SendMessage {}

/// Use this method to send photos.
/// On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
#[derive(Serialize)]
pub struct SendPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Photo to send.
    /// Pass a file_id as String to send a photo that exists on the Telegram servers (recommended),
    /// pass an HTTP URL as a String for Telegram to get a photo from the Internet,
    /// or upload a new photo using multipart/form-data.
    /// The photo must be at most 10 MB in size.
    /// The photo's width and height must not exceed 10000 in total.
    /// Width and height ratio must be at most 20.
    /// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
    pub photo: InputFileVariant,
    /// Photo caption (may also be used when resending photos by *file_id*), 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the message text.
    /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages).
    /// Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass *True*, if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options.
    /// A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating),
    /// [custom reply keyboard](https://core.telegram.org/bots#keyboards),
    /// instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendPhoto {
    /// Create a new sendPhoto request
    pub fn new(chat_id: impl Into<ChatId>, photo: impl Into<InputFileVariant>) -> Self {
        Self {
            chat_id: chat_id.into(),
            photo: photo.into(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    pub fn with_caption(self, caption: impl Into<String>) -> Self {
        Self {
            caption: Some(caption.into()),
            ..self
        }
    }
    /// Set parse mode
    pub fn with_parse_mode(self, parse_mode: ParseMode) -> Self {
        Self {
            parse_mode: Some(parse_mode),
            ..self
        }
    }
    /// Set caption entities
    pub fn with_entities(self, entities: Vec<MessageEntity>) -> Self {
        Self {
            caption_entities: Some(entities),
            ..self
        }
    }
    /// Add one entity
    pub fn with_entity(mut self, entity: MessageEntity) -> Self {
        let entities = self.caption_entities.get_or_insert_with(Default::default);
        entities.push(entity);
        self
    }
    /// Disable notification
    pub fn disable_notification(self) -> Self {
        Self {
            disable_notification: Some(true),
            ..self
        }
    }
    /// Reply to message
    pub fn reply_to(self, message_id: i64) -> Self {
        Self {
            reply_to_message_id: Some(message_id),
            ..self
        }
    }
    /// Allow sending message even if the replying message isn't present
    pub fn allow_sending_without_reply(self) -> Self {
        Self {
            allow_sending_without_reply: Some(true),
            ..self
        }
    }
    /// Set reply markup
    pub fn with_reply_markup(self, markup: impl Into<ReplyMarkup>) -> Self {
        Self {
            reply_markup: Some(markup.into()),
            ..self
        }
    }
}

impl TelegramMethod for SendPhoto {
    type Response = Message;

    fn name() -> &'static str {
        "sendPhoto"
    }
}

impl FileMethod for SendPhoto {
    fn files(&self) -> Option<HashMap<&str, &InputFile>> {
        if let InputFileVariant::File(file) = &self.photo {
            let mut map = HashMap::new();
            map.insert("photo", file);
            Some(map)
        } else {
            None
        }
    }
}

/// Use this method to send audio files, if you want Telegram clients to display them in the music player.
/// Your audio must be in the .MP3 or .M4A format.
/// On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
// Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
///
/// For sending voice messages, use the sendVoice method instead.
#[derive(Serialize)]
pub struct SendAudio {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Audio file to send.
    /// Pass a file_id as String to send an audio file that exists on the Telegram servers (recommended),
    /// pass an HTTP URL as a String for Telegram to get an audio file from the Internet,
    /// or upload a new one using multipart/form-data.
    /// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
    pub audio: InputFileVariant,
    /// Duration of the audio in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    /// Performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Track name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side.
    /// The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail's width and height should not exceed 320.
    /// Ignored if the file is not uploaded using multipart/form-data.
    /// Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileVariant>,
    /// Audio caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the message text.
    /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages).
    /// Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass *True*, if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options.
    /// A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating),
    /// [custom reply keyboard](https://core.telegram.org/bots#keyboards),
    /// instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendAudio {
    /// Create a new sendAudio request
    pub fn new(chat_id: impl Into<ChatId>, audio: impl Into<InputFileVariant>) -> Self {
        Self {
            chat_id: chat_id.into(),
            audio: audio.into(),
            duration: None,
            performer: None,
            title: None,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    /// Set duration
    pub fn with_duration(self, duration: u32) -> Self {
        Self {
            duration: Some(duration),
            ..self
        }
    }
    /// Set performer
    pub fn with_performer(self, performer: impl Into<String>) -> Self {
        Self {
            performer: Some(performer.into()),
            ..self
        }
    }
    /// Set title
    pub fn with_title(self, title: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            ..self
        }
    }
    /// Set thumbnail
    pub fn with_thumbnail(self, thumbnail: impl Into<InputFileVariant>) -> Self {
        Self {
            thumb: Some(thumbnail.into()),
            ..self
        }
    }
    /// Set caption
    pub fn with_caption(self, caption: impl Into<String>) -> Self {
        Self {
            caption: Some(caption.into()),
            ..self
        }
    }
    /// Set parse mode
    pub fn with_parse_mode(self, parse_mode: ParseMode) -> Self {
        Self {
            parse_mode: Some(parse_mode),
            ..self
        }
    }
    /// Set caption entities
    pub fn with_entities(self, entities: Vec<MessageEntity>) -> Self {
        Self {
            caption_entities: Some(entities),
            ..self
        }
    }
    /// Add one entity
    pub fn with_entity(mut self, entity: MessageEntity) -> Self {
        let entities = self.caption_entities.get_or_insert_with(Default::default);
        entities.push(entity);
        self
    }
    /// Disable notification
    pub fn disable_notification(self) -> Self {
        Self {
            disable_notification: Some(true),
            ..self
        }
    }
    /// Reply to message
    pub fn reply_to(self, message_id: i64) -> Self {
        Self {
            reply_to_message_id: Some(message_id),
            ..self
        }
    }
    /// Allow sending message even if the replying message isn't present
    pub fn allow_sending_without_reply(self) -> Self {
        Self {
            allow_sending_without_reply: Some(true),
            ..self
        }
    }
    /// Set reply markup
    pub fn with_reply_markup(self, markup: impl Into<ReplyMarkup>) -> Self {
        Self {
            reply_markup: Some(markup.into()),
            ..self
        }
    }
}

impl TelegramMethod for SendAudio {
    type Response = Message;

    fn name() -> &'static str {
        "sendAudio"
    }
}

impl FileMethod for SendAudio {
    fn files(&self) -> Option<HashMap<&str, &InputFile>> {
        let mut map = HashMap::new();
        if let InputFileVariant::File(file) = &self.audio {
            map.insert("audio", file);
        }
        if let Some(InputFileVariant::File(file)) = &self.thumb {
            map.insert("thumb", file);
        }
        if map.is_empty() {
            None
        } else {
            Some(map)
        }
    }
}

/// Use this method to send general files. On success, the sent Message is returned.
/// Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
#[derive(Serialize)]
pub struct SendDocument {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// File to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended),
    /// pass an HTTP URL as a String for Telegram to get a file from the Internet,
    /// or upload a new one using multipart/form-data.
    /// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
    pub document: InputFileVariant,
    /// Disables automatic server-side content type detection for files uploaded using multipart/form-data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side.
    /// The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail's width and height should not exceed 320.
    /// Ignored if the file is not uploaded using multipart/form-data.
    /// Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileVariant>,
    /// Document caption (may also be used when resending documents by file_id), 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the message text.
    /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages).
    /// Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass *True*, if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options.
    /// A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating),
    /// [custom reply keyboard](https://core.telegram.org/bots#keyboards),
    /// instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendDocument {
    /// Create a new sendDocument request
    pub fn new(chat_id: impl Into<ChatId>, document: impl Into<InputFileVariant>) -> Self {
        Self {
            chat_id: chat_id.into(),
            document: document.into(),
            disable_content_type_detection: None,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    /// Set thumbnail
    pub fn with_thumbnail(self, thumbnail: impl Into<InputFileVariant>) -> Self {
        Self {
            thumb: Some(thumbnail.into()),
            ..self
        }
    }
    /// Disable file type detection
    pub fn disable_content_type_detection(self) -> Self {
        Self {
            disable_content_type_detection: Some(true),
            ..self
        }
    }
    /// Set caption
    pub fn with_caption(self, caption: impl Into<String>) -> Self {
        Self {
            caption: Some(caption.into()),
            ..self
        }
    }
    /// Set parse mode
    pub fn with_parse_mode(self, parse_mode: ParseMode) -> Self {
        Self {
            parse_mode: Some(parse_mode),
            ..self
        }
    }
    /// Set caption entities
    pub fn with_entities(self, entities: Vec<MessageEntity>) -> Self {
        Self {
            caption_entities: Some(entities),
            ..self
        }
    }
    /// Add one entity
    pub fn with_entity(mut self, entity: MessageEntity) -> Self {
        let entities = self.caption_entities.get_or_insert_with(Default::default);
        entities.push(entity);
        self
    }
    /// Disable notification
    pub fn disable_notification(self) -> Self {
        Self {
            disable_notification: Some(true),
            ..self
        }
    }
    /// Reply to message
    pub fn reply_to(self, message_id: i64) -> Self {
        Self {
            reply_to_message_id: Some(message_id),
            ..self
        }
    }
    /// Allow sending message even if the replying message isn't present
    pub fn allow_sending_without_reply(self) -> Self {
        Self {
            allow_sending_without_reply: Some(true),
            ..self
        }
    }
    /// Set reply markup
    pub fn with_reply_markup(self, markup: impl Into<ReplyMarkup>) -> Self {
        Self {
            reply_markup: Some(markup.into()),
            ..self
        }
    }
}

impl TelegramMethod for SendDocument {
    type Response = Message;

    fn name() -> &'static str {
        "sendDocument"
    }
}

impl FileMethod for SendDocument {
    fn files(&self) -> Option<HashMap<&str, &InputFile>> {
        let mut map = HashMap::new();
        if let InputFileVariant::File(file) = &self.document {
            map.insert("document", file);
        }
        if let Some(InputFileVariant::File(file)) = &self.thumb {
            map.insert("thumb", file);
        }
        if map.is_empty() {
            None
        } else {
            Some(map)
        }
    }
}

/// Use this method to send video files, Telegram clients support mp4 videos (other formats may be sent as [Document](https://core.telegram.org/bots/api#document)).
/// On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
/// Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
#[derive(Serialize)]
pub struct SendVideo {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Video to send. Pass a file_id as String to send a video that exists on the Telegram servers (recommended),
    /// pass an HTTP URL as a String for Telegram to get a video from the Internet,
    /// or upload a new video using multipart/form-data.
    /// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
    pub video: InputFileVariant,
    /// Duration of sent video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    /// Pass *True*, if the uploaded video is suitable for streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side.
    /// The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail's width and height should not exceed 320.
    /// Ignored if the file is not uploaded using multipart/form-data.
    /// Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileVariant>,
    /// Video caption (may also be used when resending videos by *file_id*), 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the message text.
    /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages).
    /// Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass *True*, if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options.
    /// A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating),
    /// [custom reply keyboard](https://core.telegram.org/bots#keyboards),
    /// instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendVideo {
    /// Create a new sendVideo request
    pub fn new(chat_id: impl Into<ChatId>, video: impl Into<InputFileVariant>) -> Self {
        Self {
            chat_id: chat_id.into(),
            video: video.into(),
            duration: None,
            width: None,
            height: None,
            supports_streaming: None,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    /// Set duration
    pub fn with_duration(self, duration: u32) -> Self {
        Self {
            duration: Some(duration),
            ..self
        }
    }
    /// Set width
    pub fn with_width(self, width: u32) -> Self {
        Self {
            width: Some(width),
            ..self
        }
    }
    /// Set height
    pub fn with_height(self, height: u32) -> Self {
        Self {
            height: Some(height),
            ..self
        }
    }
    /// Set as streaming video
    pub fn set_streaming(self) -> Self {
        Self {
            supports_streaming: Some(true),
            ..self
        }
    }
    /// Set thumbnail
    pub fn with_thumbnail(self, thumbnail: impl Into<InputFileVariant>) -> Self {
        Self {
            thumb: Some(thumbnail.into()),
            ..self
        }
    }
    /// Set caption
    pub fn with_caption(self, caption: impl Into<String>) -> Self {
        Self {
            caption: Some(caption.into()),
            ..self
        }
    }
    /// Set parse mode
    pub fn with_parse_mode(self, parse_mode: ParseMode) -> Self {
        Self {
            parse_mode: Some(parse_mode),
            ..self
        }
    }
    /// Set caption entities
    pub fn with_entities(self, entities: Vec<MessageEntity>) -> Self {
        Self {
            caption_entities: Some(entities),
            ..self
        }
    }
    /// Add one entity
    pub fn with_entity(mut self, entity: MessageEntity) -> Self {
        let entities = self.caption_entities.get_or_insert_with(Default::default);
        entities.push(entity);
        self
    }
    /// Disable notification
    pub fn disable_notification(self) -> Self {
        Self {
            disable_notification: Some(true),
            ..self
        }
    }
    /// Reply to message
    pub fn reply_to(self, message_id: i64) -> Self {
        Self {
            reply_to_message_id: Some(message_id),
            ..self
        }
    }
    /// Allow sending message even if the replying message isn't present
    pub fn allow_sending_without_reply(self) -> Self {
        Self {
            allow_sending_without_reply: Some(true),
            ..self
        }
    }
    /// Set reply markup
    pub fn with_reply_markup(self, markup: impl Into<ReplyMarkup>) -> Self {
        Self {
            reply_markup: Some(markup.into()),
            ..self
        }
    }
}

impl TelegramMethod for SendVideo {
    type Response = Message;

    fn name() -> &'static str {
        "sendVideo"
    }
}

impl FileMethod for SendVideo {
    fn files(&self) -> Option<HashMap<&str, &InputFile>> {
        let mut map = HashMap::new();
        if let InputFileVariant::File(file) = &self.video {
            map.insert("audio", file);
        }
        if let Some(InputFileVariant::File(file)) = &self.thumb {
            map.insert("thumb", file);
        }
        if map.is_empty() {
            None
        } else {
            Some(map)
        }
    }
}

/// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type.
/// On success, an array of Messages that were sent is returned.
#[derive(Serialize)]
pub struct SendMediaGroup {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// A JSON-serialized array describing messages to be sent, must include 2-10 items
    pub media: Vec<InputMedia>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages).
    /// Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass *True*, if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
}

impl SendMediaGroup {
    /// Create
    pub fn new(chat_id: impl Into<ChatId>) -> Self {
        Self {
            chat_id: chat_id.into(),
            media: vec![],
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
        }
    }
    /// Set media group
    pub fn with_media_group(self, media_group: Vec<InputMedia>) -> Self {
        Self {
            media: media_group,
            ..self
        }
    }
    /// Add one media file
    pub fn with_media(mut self, media: impl Into<InputMedia>) -> Self {
        self.media.push(media.into());
        self
    }
    /// Disable notification
    pub fn disable_notification(self) -> Self {
        Self {
            disable_notification: Some(true),
            ..self
        }
    }
    /// Reply to message
    pub fn reply_to(self, message_id: i64) -> Self {
        Self {
            reply_to_message_id: Some(message_id),
            ..self
        }
    }
    /// Allow sending message even if the replying message isn't present
    pub fn allow_sending_without_reply(self) -> Self {
        Self {
            allow_sending_without_reply: Some(true),
            ..self
        }
    }
}

impl TelegramMethod for SendMediaGroup {
    type Response = Vec<Message>;

    fn name() -> &'static str {
        "sendMediaGroup"
    }
}
