use serde::{Deserialize, Serialize};

use crate::markup::{MessageEntity, ParseMode};
use crate::{JsonMethod, TelegramMethod};

/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(Deserialize)]
pub struct Animation {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width as defined by sender
    pub width: usize,
    /// Video height as defined by sender
    pub height: usize,
    /// Duration of the video in seconds as defined by sender
    pub duration: u32,
    /// Animation thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Original animation filename as defined by sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<usize>,
}

/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Deserialize)]
pub struct Audio {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: u32,
    /// Performer of the audio as defined by sender or by audio tags
    pub performer: Option<String>,
    /// Title of the audio as defined by sender or by audio tags
    pub title: Option<String>,
    /// Original filename as defined by sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<usize>,
    /// Thumbnail of the album cover to which the music file belongs
    pub thumb: Option<PhotoSize>,
}

/// This object represents a general file (as opposed to
/// [photos](https://core.telegram.org/bots/api#photosize),
/// [voice messages](https://core.telegram.org/bots/api#voice) and
/// [audio files](https://core.telegram.org/bots/api#audio)).
#[derive(Deserialize)]
pub struct Document {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Document thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Original filename as defined by sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<usize>,
}

/// This object represents one size of a photo or a
/// [file](https://core.telegram.org/bots/api#document) /
/// [sticker](https://core.telegram.org/bots/api#sticker) thumbnail.
#[derive(Deserialize)]
pub struct PhotoSize {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Photo width
    pub width: u32,
    /// Photo height
    pub height: u32,
    /// File size
    pub file_size: u32,
}

/// This object represents a video file.
#[derive(Deserialize)]
pub struct Video {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width as defined by sender
    pub width: u32,
    /// Video height as defined by sender
    pub height: u32,
    /// Duration of the video in seconds as defined by sender
    pub duration: u32,
    /// Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// Original animation filename as defined by sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<u32>,
}

/// This object represents a [video message](https://telegram.org/blog/video-messages-and-telescope)
/// (available in Telegram apps as of [v.4.0](https://telegram.org/blog/video-messages-and-telescope)).
#[derive(Deserialize)]
pub struct VideoNote {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width and height (diameter of the video message) as defined by sender
    pub length: u32,
    /// Duration of the video in seconds as defined by sender
    pub duration: u32,
    /// Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// File size
    pub file_size: Option<u32>,
}

/// This object represents a voice note.
#[derive(Deserialize)]
pub struct Voice {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: u32,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<u32>,
}

/// This object represents a file ready to be downloaded.
///
/// The file can be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`.
/// It is guaranteed that the link will be valid for at least 1 hour.
/// When the link expires, a new one can be requested by calling [getFile](https://core.telegram.org/bots/api#getfile).
#[derive(Deserialize)]
pub struct File {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// File size, if known
    pub file_size: Option<u32>,
    /// File path. Use `https://api.telegram.org/file/bot<token>/<file_path>` to get the file.
    pub file_path: Option<String>,
}

/// This object represents the content of a media message to be sent.
/// It should be one of
/// - InputMediaAnimation
/// - InputMediaDocument
/// - InputMediaAudio
/// - InputMediaPhoto
/// - InputMediaVideo
#[derive(Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum InputMedia {
    /// Represents a photo to be sent.
    Photo {
        /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended),
        /// pass an HTTP URL for Telegram to get a file from the Internet,
        /// or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name.
        //// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
        media: String,
        /// Caption of the photo to be sent, 0-1024 characters after entities parsing
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Mode for parsing entities in the photo caption.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in the caption,
        /// which can be specified instead of parse_mode
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
    },
    /// Represents a video to be sent.
    Video {
        /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended),
        /// pass an HTTP URL for Telegram to get a file from the Internet,
        /// or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name.
        //// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
        media: String,
        /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side.
        /// The thumbnail should be in JPEG format and less than 200 kB in size.
        /// A thumbnail's width and height should not exceed 320.
        /// Ignored if the file is not uploaded using multipart/form-data.
        /// Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>.
        /// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<InputFileVariant>,
        /// Video width
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u32>,
        /// Video height
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<u32>,
        /// Video duration
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<u32>,
        /// Pass True, if the uploaded video is suitable for streaming
        supports_streaming: Option<bool>,
        /// Caption of the video to be sent, 0-1024 characters after entities parsing
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Mode for parsing entities in the video caption.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in the caption,
        /// which can be specified instead of *parse_mode*
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
    },
    /// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
    Animation {
        /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended),
        /// pass an HTTP URL for Telegram to get a file from the Internet,
        /// or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name.
        //// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
        media: String,
        /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side.
        /// The thumbnail should be in JPEG format and less than 200 kB in size.
        /// A thumbnail's width and height should not exceed 320.
        /// Ignored if the file is not uploaded using multipart/form-data.
        /// Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>.
        /// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<InputFileVariant>,
        /// Animation width
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u32>,
        /// Animation height
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<u32>,
        /// Animation duration
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<u32>,
        /// Caption of the animation to be sent, 0-1024 characters after entities parsing
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Mode for parsing entities in the animation caption.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in the caption,
        /// which can be specified instead of *parse_mode*
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
    },
    /// Represents an audio file to be treated as music to be sent.
    Audio {
        /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended),
        /// pass an HTTP URL for Telegram to get a file from the Internet,
        /// or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name.
        //// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
        media: String,
        /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side.
        /// The thumbnail should be in JPEG format and less than 200 kB in size.
        /// A thumbnail's width and height should not exceed 320.
        /// Ignored if the file is not uploaded using multipart/form-data.
        /// Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>.
        /// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<InputFileVariant>,
        /// Performer of the audio
        #[serde(skip_serializing_if = "Option::is_none")]
        performer: Option<String>,
        /// Title of the audio
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        /// Duration of the audio in seconds
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<u32>,
        /// Caption of the audio to be sent, 0-1024 characters after entities parsing
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Mode for parsing entities in the audio caption.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in the caption,
        /// which can be specified instead of *parse_mode*
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
    },
    /// Represents a general file to be sent.
    Document {
        /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended),
        /// pass an HTTP URL for Telegram to get a file from the Internet,
        /// or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name.
        //// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
        media: String,
        /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side.
        /// The thumbnail should be in JPEG format and less than 200 kB in size.
        /// A thumbnail's width and height should not exceed 320.
        /// Ignored if the file is not uploaded using multipart/form-data.
        /// Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>.
        /// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<InputFileVariant>,
        /// Caption of the document to be sent, 0-1024 characters after entities parsing
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Mode for parsing entities in the document caption.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in the caption,
        /// which can be specified instead of *parse_mode*
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
    },
}

/// Thumbnail type.
#[derive(Serialize)]
#[serde(untagged)]
pub enum InputFileVariant {
    /// Use existing file
    File(InputFile),
    /// Upload a new file
    Id(String),
}

impl From<InputFile> for InputFileVariant {
    fn from(file: InputFile) -> Self {
        Self::File(file)
    }
}

impl From<String> for InputFileVariant {
    fn from(id: String) -> Self {
        Self::Id(id)
    }
}

impl From<&str> for InputFileVariant {
    fn from(id: &str) -> Self {
        Self::Id(id.to_string())
    }
}

pub struct InputFile {
    pub name: String,
    pub data: Vec<u8>,
    pub mime: String,
}

impl Serialize for InputFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        "".serialize(serializer)
    }
}

/// Use this method to get basic info about a file and prepare it for downloading.
/// For the moment, bots can download files of up to 20MB in size.
/// On success, a [File](https://core.telegram.org/bots/api#file) object is returned.
/// The file can then be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`, where `<file_path>` is taken from the response.
/// It is guaranteed that the link will be valid for at least 1 hour.
/// When the link expires, a new one can be requested by calling [getFile](https://core.telegram.org/bots/api#getfile) again.
///
/// **Note:** This function may not preserve the original file name and MIME type.
/// You should save the file's MIME type and name (if available) when the File object is received.
#[derive(Serialize)]
pub struct GetFile {
    /// File identifier to get info about
    pub file_id: String,
}

impl GetFile {
    /// Create a new getFile request
    pub fn new(file_id: impl Into<String>) -> Self {
        Self {
            file_id: file_id.into(),
        }
    }
}

impl TelegramMethod for GetFile {
    type Response = File;

    fn name() -> &'static str {
        "getFile"
    }
}

impl JsonMethod for GetFile {}
