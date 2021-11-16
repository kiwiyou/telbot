use crate::{
    chat::ChatId,
    file::{InputFileVariant, PhotoSize},
    markup::ReplyMarkup,
    message::Message,
    JsonMethod, TelegramMethod,
};
use serde::{Deserialize, Serialize};

/// This object represents a sticker.
#[derive(Debug, Deserialize)]
pub struct Sticker {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Sticker width
    pub width: u32,
    /// Sticker height
    pub height: u32,
    /// *True*, if the sticker is [animated](https://telegram.org/blog/animated-stickers)
    pub is_animated: bool,
    /// Sticker thumbnail in the .WEBP or .JPG format
    pub thumb: Option<PhotoSize>,
    /// Emoji associated with the sticker
    pub emoji: Option<String>,
    /// Name of the sticker set to which the sticker belongs
    pub set_name: Option<String>,
    /// For mask stickers, the position where the mask should be placed
    pub mask_position: Option<MaskPosition>,
    /// File size
    pub file_size: Option<u32>,
}

/// This object represents a sticker set.
#[derive(Debug, Deserialize)]
pub struct StickerSet {
    /// Sticker set name
    pub name: String,
    /// Sticker set title
    pub title: String,
    /// *True*, if the sticker set contains [animated stickers](https://telegram.org/blog/animated-stickers)
    pub is_animated: bool,
    /// *True*, if the sticker set contains masks
    pub contains_masks: bool,
    /// List of all set stickers
    pub stickers: Vec<Sticker>,
    /// Sticker set thumbnail in the .WEBP or .TGS format
    pub thumb: Option<PhotoSize>,
}

/// This object describes the position on faces where a mask should be placed by default.
#[derive(Debug, Deserialize)]
pub struct MaskPosition {
    /// The part of the face relative to which the mask should be placed.
    /// One of “forehead”, “eyes”, “mouth”, or “chin”.
    pub point: MaskPoint,
    /// Shift by X-axis measured in widths of the mask scaled to the face size, from left to right.
    /// For example, choosing -1.0 will place mask just to the left of the default mask position.
    pub x_shift: f32,
    /// Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom.
    /// For example, 1.0 will place the mask just below the default mask position.
    pub y_shift: f32,
    /// Mask scaling coefficient. For example, 2.0 means double size.
    pub scale: f32,
}

/// The part of the face used in masked stickers.
#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub enum MaskPoint {
    Forehead,
    Eyes,
    Mouth,
    Chin,
}

/// Use this method to send static .WEBP or [animated](https://telegram.org/blog/animated-stickers) .TGS stickers.
/// On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
#[derive(Clone, Serialize)]
pub struct SendSticker {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Sticker to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended),
    /// pass an HTTP URL as a String for Telegram to get a .WEBP file from the Internet,
    /// or upload a new one using multipart/form-data.
    /// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
    pub sticker: InputFileVariant,
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

impl SendSticker {
    /// Create a new sendSticker request
    pub fn new(chat_id: impl Into<ChatId>, sticker: impl Into<InputFileVariant>) -> Self {
        Self {
            chat_id: chat_id.into(),
            sticker: sticker.into(),
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
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

impl TelegramMethod for SendSticker {
    type Response = Message;

    fn name() -> &'static str {
        "sendSticker"
    }
}

impl JsonMethod for SendSticker {}

/// Use this method to get a sticker set. On success, a [StickerSet](https://core.telegram.org/bots/api#stickerset) object is returned.
#[derive(Clone, Serialize)]
pub struct GetStickerSet {
    /// Name of the sticker set
    pub name: String,
}

impl GetStickerSet {
    /// Create a new getStickerSet request
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl TelegramMethod for GetStickerSet {
    type Response = StickerSet;

    fn name() -> &'static str {
        "getStickerSet"
    }
}

impl JsonMethod for GetStickerSet {}
