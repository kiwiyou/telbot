use std::collections::HashMap;

use crate::{
    chat::ChatId,
    file::{File, InputFile, InputFileVariant, PhotoSize},
    markup::ReplyMarkup,
    message::Message,
    FileMethod, JsonMethod, TelegramMethod,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

/// Use this method to upload a .PNG file with a sticker for later use
/// in *createNewStickerSet* and *addStickerToSet* methods (can be used multiple times).
/// Returns the uploaded [`File`] on success.
#[derive(Clone, Serialize)]
pub struct UploadStickerFile {
    /// User identifier of sticker file owner
    pub user_id: i64,
    /// **PNG** image with the sticker, must be up to 512 kilobytes in size,
    /// dimensions must not exceed 512px, and either width or height must be exactly 512px.
    /// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
    pub png_sticker: InputFile,
}

impl UploadStickerFile {
    /// Create a new uploadStickerFile request
    pub fn new(user_id: i64, png_sticker: InputFile) -> Self {
        Self {
            user_id,
            png_sticker,
        }
    }
}

impl TelegramMethod for UploadStickerFile {
    type Response = File;

    fn name() -> &'static str {
        "uploadStickerFile"
    }
}

impl FileMethod for UploadStickerFile {
    fn files(&self) -> Option<std::collections::HashMap<&str, &InputFile>> {
        let mut map = HashMap::new();
        map.insert("png_sticker", &self.png_sticker);
        Some(map)
    }
}

/// Use this method to create a new sticker set owned by a user.
/// The bot will be able to edit the sticker set thus created.
/// You must use exactly one of the fields *png_sticker* or *tgs_sticker*.
/// Returns *True* on success.
#[derive(Clone, Serialize)]
pub struct CreateNewStickerSet {
    /// User identifier of created sticker set owner.
    pub user_id: i64,
    /// Short name of sticker set, to be used in `t.me/addstickers/` URLs (e.g., *animals*).
    /// Can contain only english letters, digits and underscores.
    /// Must begin with a letter, can't contain consecutive underscores and must end in *“_by_<bot username>”*.
    /// *<bot_username>* is case insensitive. 1-64 characters.
    pub name: String,
    /// Sticker set title, 1-64 characters.
    pub title: String,
    /// **PNG** image with the sticker, must be up to 512 kilobytes in size,
    /// dimensions must not exceed 512px, and either width or height must be exactly 512px.
    /// Pass a *file_id* as a String to send a file that already exists on the Telegram servers,
    /// pass an HTTP URL as a String for Telegram to get a file from the Internet,
    /// or upload a new one using multipart/form-data.
    /// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub png_sticker: Option<InputFileVariant>,
    /// **TGS** animation with the sticker, uploaded using multipart/form-data.
    /// See https://core.telegram.org/animated_stickers#technical-requirements for technical requirements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgs_sticker: Option<InputFile>,
    /// One or more emoji corresponding to the sticker.
    pub emojis: String,
    /// Pass *True*, if a set of mask stickers should be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_masks: Option<bool>,
    /// A JSON-serialized object for position where the mask should be placed on faces.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

impl CreateNewStickerSet {
    /// Create a new createNewStickerSet request with png sticker
    pub fn new_png(
        user_id: i64,
        name: impl Into<String>,
        title: impl Into<String>,
        emojis: impl Into<String>,
        png_sticker: impl Into<InputFileVariant>,
    ) -> Self {
        Self {
            user_id,
            name: name.into(),
            title: title.into(),
            png_sticker: Some(png_sticker.into()),
            tgs_sticker: None,
            emojis: emojis.into(),
            contains_masks: None,
            mask_position: None,
        }
    }
    /// Create a new createNewStickerSet request with tgs sticker
    pub fn new_tgs(
        user_id: i64,
        name: impl Into<String>,
        title: impl Into<String>,
        emojis: impl Into<String>,
        tgs_sticker: InputFile,
    ) -> Self {
        Self {
            user_id,
            name: name.into(),
            title: title.into(),
            png_sticker: None,
            tgs_sticker: Some(tgs_sticker),
            emojis: emojis.into(),
            contains_masks: None,
            mask_position: None,
        }
    }
    /// Mark as mask sticker
    pub fn with_masks(self) -> Self {
        Self {
            contains_masks: Some(true),
            ..self
        }
    }
    /// Set mask position
    pub fn with_mask_position(self, position: MaskPosition) -> Self {
        Self {
            mask_position: Some(position),
            ..self
        }
    }
}

impl TelegramMethod for CreateNewStickerSet {
    type Response = bool;

    fn name() -> &'static str {
        "createNewStickerSet"
    }
}

impl FileMethod for CreateNewStickerSet {
    fn files(&self) -> Option<HashMap<&str, &InputFile>> {
        let mut map = HashMap::new();
        match (&self.png_sticker, &self.tgs_sticker) {
            (None, Some(tgs)) => {
                map.insert("tgs_sticker", tgs);
            },
            (Some(InputFileVariant::File(png)), None) => {
                map.insert("png_sticker", png);
            }
            (Some(InputFileVariant::Id(_)), None) => {},
            _ => panic!("exactly one of CreateNewStickerSet::png_sticker or CreateNewStickerSet::tgs_sticker can be used"),
        }
        Some(map)
    }
}

/// Use this method to add a new sticker to a set created by the bot.
/// You **must** use exactly one of the fields _png_sticker_ or _tgs_sticker_.
/// Animated stickers can be added to animated sticker sets and only to them.
/// Animated sticker sets can have up to 50 stickers
/// Static sticker sets can have up to 120 stickers.
/// Returns _True_ on success.
#[derive(Clone, Serialize)]
pub struct AddStickerToSet {
    /// User identifier of sticker file owner
    pub user_id: i64,
    /// Sticker set name
    pub name: String,
    /// **PNG** image with the sticker, must be up to 512 kilobytes in size,
    /// dimensions must not exceed 512px, and either width or height must be exactly 512px.
    /// Pass a *file_id* as a String to send a file that already exists on the Telegram servers,
    /// pass an HTTP URL as a String for Telegram to get a file from the Internet,
    /// or upload a new one using multipart/form-data.
    /// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub png_sticker: Option<InputFileVariant>,
    /// **TGS** animation with the sticker, uploaded using multipart/form-data.
    /// See https://core.telegram.org/animated_stickers#technical-requirements for technical requirements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgs_sticker: Option<InputFile>,
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    /// A JSON-serialized object for position where the mask should be placed on faces
    pub mask_position: Option<MaskPosition>,
}

impl AddStickerToSet {
    /// Create a new addStickerToSet request with png sticker
    pub fn new_png(
        user_id: i64,
        name: impl Into<String>,
        emojis: impl Into<String>,
        png_sticker: impl Into<InputFileVariant>,
    ) -> Self {
        Self {
            user_id,
            name: name.into(),
            png_sticker: Some(png_sticker.into()),
            tgs_sticker: None,
            emojis: emojis.into(),
            mask_position: None,
        }
    }
    /// Create a new addStickerToSet request with tgs sticker
    pub fn new_tgs(
        user_id: i64,
        name: impl Into<String>,
        emojis: impl Into<String>,
        tgs_sticker: InputFile,
    ) -> Self {
        Self {
            user_id,
            name: name.into(),
            png_sticker: None,
            tgs_sticker: Some(tgs_sticker),
            emojis: emojis.into(),
            mask_position: None,
        }
    }
    /// Set mask position
    pub fn with_mask_position(self, position: MaskPosition) -> Self {
        Self {
            mask_position: Some(position),
            ..self
        }
    }
}

impl TelegramMethod for AddStickerToSet {
    type Response = bool;

    fn name() -> &'static str {
        "addStickerToSet"
    }
}

impl FileMethod for AddStickerToSet {
    fn files(&self) -> Option<HashMap<&str, &InputFile>> {
        let mut map = HashMap::new();
        match (&self.png_sticker, &self.tgs_sticker) {
            (None, Some(tgs)) => {
                map.insert("tgs_sticker", tgs);
            },
            (Some(InputFileVariant::File(png)), None) => {
                map.insert("png_sticker", png);
            }
            (Some(InputFileVariant::Id(_)), None) => {},
            _ => panic!("exactly one of AddStickerToSet::png_sticker or AddStickerToSet::tgs_sticker can be used"),
        }
        Some(map)
    }
}

/// Use this method to move a sticker in a set created by the bot to a specific position.
/// Returns _True_ on success.
#[derive(Clone, Serialize)]
pub struct SetStickerPositionInSet {
    pub sticker: String,
    pub position: usize,
}

impl SetStickerPositionInSet {
    /// Create a new setStickerPositionSet request
    pub fn new(sticker: impl Into<String>, position: usize) -> Self {
        Self {
            sticker: sticker.into(),
            position,
        }
    }
}

impl TelegramMethod for SetStickerPositionInSet {
    type Response = bool;

    fn name() -> &'static str {
        "setStickerPositionInSet"
    }
}

impl JsonMethod for SetStickerPositionInSet {}

/// Use this method to delete a sticker from a set created by the bot.
/// Returns _True_ on success.
#[derive(Clone, Serialize)]
pub struct DeleteStickerFromSet {
    pub sticker: String,
}

impl DeleteStickerFromSet {
    /// Create a new deleteStickerFromSet request
    pub fn new(sticker: impl Into<String>) -> Self {
        Self {
            sticker: sticker.into(),
        }
    }
}

impl TelegramMethod for DeleteStickerFromSet {
    type Response = bool;

    fn name() -> &'static str {
        "deleteStickerFromSet"
    }
}

impl JsonMethod for DeleteStickerFromSet {}

/// Use this method to set the thumbnail of a sticker set.
/// Animated thumbnails can be set for animated sticker sets only.
/// Returns _True_ on success.
#[derive(Clone, Serialize)]
pub struct SetStickerSetThumb {
    /// Sticker set name
    pub name: String,
    /// User identifier of the sticker set owner
    pub user_id: i64,
    /// A **PNG** image with the thumbnail, must be up to 128 kilobytes in size
    /// and have width and height exactly 100px, or a **TGS** animation with the thumbnailup to 32 kilobytes in size;
    /// see https://core.telegram.org/animated_stickers#technical-requirements
    /// for animated sticker technical requirements.
    /// Pass a _file_id_ as a String to send a file that already exists on the Telegram servers,
    /// pass an HTTP URL as a String for Telegram to get a file from the Internet,
    /// or upload a new one using multipart/form-data.
    /// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files).
    /// Animated sticker set thumbnail can't be uploaded via HTTP URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileVariant>,
}

impl SetStickerSetThumb {
    /// Create a new setStickerSetThumb request
    pub fn new(name: impl Into<String>, user_id: i64) -> Self {
        Self {
            name: name.into(),
            user_id,
            thumb: None,
        }
    }

    /// Set thumb
    pub fn with_thumb(self, thumb: impl Into<InputFileVariant>) -> Self {
        Self {
            thumb: Some(thumb.into()),
            ..self
        }
    }
}

impl TelegramMethod for SetStickerSetThumb {
    type Response = bool;

    fn name() -> &'static str {
        "setStickerSetThumb"
    }
}

impl FileMethod for SetStickerSetThumb {
    fn files(&self) -> Option<HashMap<&str, &InputFile>> {
        if let Some(InputFileVariant::File(thumb)) = &self.thumb {
            let mut map = HashMap::new();
            map.insert("thumb", thumb);
            Some(map)
        } else {
            None
        }
    }
}
