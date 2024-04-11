use serde::{Deserialize, Serialize};

use crate::markup::{InlineKeyboardMarkup, MessageEntity, ParseMode};
use crate::message::{Location, Message};
use crate::payment::LabeledPrice;
use crate::user::User;
use crate::{JsonMethod, TelegramMethod};

/// Incoming inline query.
///
/// When the user sends an empty query, your bot could return some default or trending results.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#inlinequery)
#[derive(Debug, Deserialize)]
pub struct InlineQuery {
    /// Unique identifier for this query.
    pub id: String,
    /// Sender.
    pub from: User,
    /// Text of the query. (up to 256 characters)
    pub query: String,
    /// Offset of the results to be returned, can be controlled by the bot.
    pub offset: String,
    /// Type of the chat, from which the inline query was sent.
    ///
    /// The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat.
    pub chat_type: Option<String>,
    /// Sender location, only for bots that request user location.
    pub location: Option<Location>,
}

#[derive(Debug, Deserialize)]
pub struct ChosenInlineResult {}

/// An incoming callback query from a callback button in an
/// [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating).
///
/// If the button that originated the query was attached to a message sent by the bot, the field *message* will be present.
/// If the button was attached to a message sent via the bot
/// (in [inline mode](https://core.telegram.org/bots/api#inline-mode)), the field *inline_message_id* will be present.
/// Exactly one of the fields *data* or *game_short_name* will be present.
///
/// > **NOTE:** After the user presses a callback button,
/// > Telegram clients will display a progress bar until you call [answerCallbackQuery](https://core.telegram.org/bots/api#answercallbackquery).
/// > It is, therefore, necessary to react by calling [answerCallbackQuery](https://core.telegram.org/bots/api#answercallbackquery)
/// > even if no notification to the user is needed (e.g., without specifying any of the optional parameters).
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#callbackquery)
#[derive(Debug, Deserialize)]
pub struct CallbackQuery {
    /// Unique identifier for this query.
    pub id: String,
    /// Sender.
    pub from: User,
    /// Message with the callback button that originated the query.
    /// Note that message content and message date will not be available if the message is too old
    pub message: Option<Message>,
    /// Identifier of the message sent via the bot in inline mode, that originated the query.
    pub inline_message_id: Option<String>,
    /// Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent.
    /// Useful for high scores in [games](https://core.telegram.org/bots/api#games).
    pub chat_instance: String,
    /// Data associated with the callback button.
    /// Be aware that a bad client can send arbitrary data in this field.
    pub data: Option<String>,
    /// Short name of a [Game](https://core.telegram.org/bots/api#games) to be returned,
    /// serves as the unique identifier for the game.
    pub game_short_name: Option<String>,
}

/// One result of an inline query.
///
/// Telegram clients currently support results of the following 20 types:
///
/// - [InlineQueryResultCachedAudio](https://core.telegram.org/bots/api#inlinequeryresultcachedaudio)
/// - [InlineQueryResultCachedDocument](https://core.telegram.org/bots/api#inlinequeryresultcacheddocument)
/// - [InlineQueryResultCachedGif](https://core.telegram.org/bots/api#inlinequeryresultcachedgif)
/// - [InlineQueryResultCachedMpeg4Gif](https://core.telegram.org/bots/api#inlinequeryresultcachedmpeg4gif)
/// - [InlineQueryResultCachedPhoto](https://core.telegram.org/bots/api#inlinequeryresultcachedphoto)
/// - [InlineQueryResultCachedSticker](https://core.telegram.org/bots/api#inlinequeryresultcachedsticker)
/// - [InlineQueryResultCachedVideo](https://core.telegram.org/bots/api#inlinequeryresultcachedvideo)
/// - [InlineQueryResultCachedVoice](https://core.telegram.org/bots/api#inlinequeryresultcachedvoice)
/// - [InlineQueryResultArticle](https://core.telegram.org/bots/api#inlinequeryresultarticle)
/// - [InlineQueryResultAudio](https://core.telegram.org/bots/api#inlinequeryresultaudio)
/// - [InlineQueryResultContact](https://core.telegram.org/bots/api#inlinequeryresultcontact)
/// - [InlineQueryResultGame](https://core.telegram.org/bots/api#inlinequeryresultgame)
/// - [InlineQueryResultDocument](https://core.telegram.org/bots/api#inlinequeryresultdocument)
/// - [InlineQueryResultGif](https://core.telegram.org/bots/api#inlinequeryresultgif)
/// - [InlineQueryResultLocation](https://core.telegram.org/bots/api#inlinequeryresultlocation)
/// - [InlineQueryResultMpeg4Gif](https://core.telegram.org/bots/api#inlinequeryresultmpeg4gif)
/// - [InlineQueryResultPhoto](https://core.telegram.org/bots/api#inlinequeryresultphoto)
/// - [InlineQueryResultVenue](https://core.telegram.org/bots/api#inlinequeryresultvenue)
/// - [InlineQueryResultVideo](https://core.telegram.org/bots/api#inlinequeryresultvideo)
/// - [InlineQueryResultVoice](https://core.telegram.org/bots/api#inlinequeryresultvoice)
///
/// **Note:** All URLs passed in inline query results will be available to end users
/// and therefore must be assumed to be **public**.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#inlinequeryresult)
#[derive(Clone, Serialize)]
pub struct InlineQueryResult {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,
    /// Result type, should be handled manually.
    r#type: &'static str,
    /// Result type.
    #[serde(flatten)]
    pub kind: InlineQueryResultKind,
    /// [Inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating) attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl InlineQueryResult {
    /// Sets reply markup.
    pub fn with_reply_markup(self, markup: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(markup.into()),
            ..self
        }
    }
}

/// Type of inline query result.
#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum InlineQueryResultKind {
    /// A link to an article or web page.
    Article {
        /// Title of the result.
        title: String,
        /// Content of the message to be sent.
        input_message_content: InputMessageContent,
        /// URL of the result.
        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,
        /// Pass `true`, if you don't want the URL to be shown in the message.
        #[serde(skip_serializing_if = "Option::is_none")]
        hide_url: Option<bool>,
        /// Short description of the result.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Url of the thumbnail for the result.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_url: Option<String>,
        /// Thumbnail width.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_width: Option<u32>,
        /// Thumbnail height.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_height: Option<u32>,
    },
    /// A link to a photo.
    ///
    /// By default, this photo will be sent by the user with optional caption.
    /// Alternatively, you can use *input_message_content* to send a message with the specified content instead of the photo.
    Photo {
        /// A valid URL of the photo. Photo must be in **jpeg** format.
        ///
        /// Photo size must not exceed 5MB.
        photo_url: String,
        /// URL of the thumbnail for the photo.
        thumb_url: String,
        /// Width of the photo.
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_width: Option<u32>,
        /// Height of the photo.
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_height: Option<u32>,
        /// Title for the result.
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        /// Short description of the result.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Caption of the photo to be sent, 0-1024 characters after entities parsing.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Mode for parsing entities in the photo caption.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in the caption,
        /// which can be specified instead of parse_mode.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        /// Content of the message to be sent instead of the photo.
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    /// A link to an animated GIF file.
    ///
    /// By default, this animated GIF file will be sent by the user with optional caption.
    /// Alternatively, you can use *input_message_content* to send a message with the specified content instead of the animation.
    Gif {
        /// A valid URL for the GIF file. File size must not exceed 1MB.
        gif_url: String,
        /// Width of the GIF.
        #[serde(skip_serializing_if = "Option::is_none")]
        gif_width: Option<u32>,
        /// Height of the GIF.
        #[serde(skip_serializing_if = "Option::is_none")]
        gif_height: Option<u32>,
        /// Duration of the GIF.
        #[serde(skip_serializing_if = "Option::is_none")]
        gif_duration: Option<u32>,
        /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result.
        thumb_url: String,
        /// MIME type of the thumbnail,
        /// must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_mime_type: Option<String>,
        /// Title for the result.
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        /// Caption of the GIF file to be sent, 0-1024 characters after entities parsing.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Mode for parsing entities in the caption.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in the caption,
        /// which can be specified instead of parse_mode.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        /// Content of the message to be sent instead of the GIF animation.
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    /// A link to a video animation (H.264/MPEG-4 AVC video without sound).
    ///
    /// By default, this animated MPEG-4 file will be sent by the user with optional caption.
    /// Alternatively, you can use *input_message_content* to send a message with the specified content instead of the animation.
    Mpeg4Gif {
        /// A valid URL for the MP4 file. File size must not exceed 1MB.
        mpeg4_url: String,
        /// Video width.
        #[serde(skip_serializing_if = "Option::is_none")]
        mpeg4_width: Option<u32>,
        /// Video height.
        #[serde(skip_serializing_if = "Option::is_none")]
        mpeg4_height: Option<u32>,
        /// Video duration.
        #[serde(skip_serializing_if = "Option::is_none")]
        mpeg4_duration: Option<u32>,
        /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result.
        thumb_url: String,
        /// MIME type of the thumbnail,
        /// must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_mime_type: Option<String>,
        /// Title for the result.
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        /// Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Mode for parsing entities in the caption.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in the caption,
        /// which can be specified instead of parse_mode.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        /// Content of the message to be sent instead of the video animation.
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    /// A link to a page containing an embedded video player or a video file.
    ///
    /// By default, this video file will be sent by the user with an optional caption.
    /// Alternatively, you can use *input_message_content* to send a message with the specified content instead of the video.
    /// > If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube),
    /// > you **must** replace its content using *input_message_content*.
    Video {
        /// A valid URL for the embedded video player or video file.
        video_url: String,
        /// Mime type of the content of video url, “text/html” or “video/mp4”.
        mime_type: String,
        /// URL of the thumbnail (jpeg only) for the video.
        thumb_url: String,
        /// Title for the result.
        title: String,
        /// Video width.
        #[serde(skip_serializing_if = "Option::is_none")]
        video_width: Option<u32>,
        /// Video height.
        #[serde(skip_serializing_if = "Option::is_none")]
        video_height: Option<u32>,
        /// Video duration in seconds.
        #[serde(skip_serializing_if = "Option::is_none")]
        video_duration: Option<u32>,
        /// Short description of the result.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Caption of the video to be sent, 0-1024 characters after entities parsing.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Mode for parsing entities in the caption.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in the caption,
        /// which can be specified instead of parse_mode.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        /// Content of the message to be sent instead of the video.
        /// This field is **required** if InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    /// A link to an MP3 audio file.
    ///
    /// By default, this audio file will be sent by the user.
    /// Alternatively, you can use *input_message_content* to send a message with the specified content instead of the audio.
    Audio {
        /// A valid URL for the audio file.
        audio_url: String,
        /// Title.
        title: String,
        /// Performer.
        performer: Option<String>,
        /// Audio duration in seconds.
        #[serde(skip_serializing_if = "Option::is_none")]
        audio_duration: Option<u32>,
        /// Caption, 0-1024 characters after entities parsing.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Mode for parsing entities in the caption.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in the caption,
        /// which can be specified instead of parse_mode.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        /// Content of the message to be sent instead of the audio.
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    /// A link to a voice recording in an .OGG container encoded with OPUS.
    ///
    /// By default, this voice recording will be sent by the user.
    /// Alternatively, you can use *input_message_content* to send a message with the specified content instead of the the voice message.
    Voice {
        /// A valid URL for the voice recording.
        voice_url: String,
        /// Recording title.
        title: String,
        /// Recording duration in seconds.
        #[serde(skip_serializing_if = "Option::is_none")]
        voice_duration: Option<u32>,
        /// Caption, 0-1024 characters after entities parsing.
        caption: Option<String>,
        /// Mode for parsing entities in the caption.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in the caption,
        /// which can be specified instead of parse_mode.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        /// Content of the message to be sent instead of the voice recording.
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    /// A link to a file.
    ///
    /// By default, this file will be sent by the user with an optional caption.
    /// Alternatively, you can use *input_message_content* to send a message with the specified content instead of the file.
    /// Currently, only **.PDF** and **.ZIP** files can be sent using this method.
    ///
    /// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    Document {
        /// A valid URL for the file.
        document_url: String,
        /// Mime type of the content of the file, either “application/pdf” or “application/zip”.
        mime_type: String,
        /// Short description of the result.
        description: String,
        /// URL of the thumbnail (jpeg only) for the file.
        thumb_url: Option<String>,
        /// Thumbnail width.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_width: Option<u32>,
        /// Thumbnail height.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_height: Option<u32>,
        /// Caption of the document to be sent, 0-1024 characters after entities parsing.
        caption: Option<String>,
        /// Mode for parsing entities in the caption.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in the caption,
        /// which can be specified instead of parse_mode.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        /// Content of the message to be sent instead of the file.
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    /// A location on a map.
    ///
    /// By default, the location will be sent by the user.
    /// Alternatively, you can use *input_message_content* to send a message with the specified content instead of the location.
    ///
    /// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    Location {
        /// Location latitude in degrees.
        latitude: f32,
        /// Location longitude in degrees.
        longitude: f32,
        /// Location title.
        title: String,
        /// The radius of uncertainty for the location, measured in meters; 0-1500.
        horizontal_accuracy: f32,
        /// Period in seconds for which the location can be updated, should be between 60 and 86400.
        #[serde(skip_serializing_if = "Option::is_none")]
        live_period: Option<u32>,
        /// For live locations, a direction in which the user is moving, in degrees.
        /// Must be between 1 and 360 if specified.
        #[serde(skip_serializing_if = "Option::is_none")]
        heading: Option<u32>,
        /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters.
        /// Must be between 1 and 100000 if specified.
        #[serde(skip_serializing_if = "Option::is_none")]
        proximity_alert_radius: Option<u32>,
        /// Url of the thumbnail for the result.
        thumb_url: Option<String>,
        /// Thumbnail width.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_width: Option<u32>,
        /// Thumbnail height.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_height: Option<u32>,
        /// Content of the message to be sent instead of the location.
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    /// A venue.
    ///
    /// By default, the venue will be sent by the user.
    /// Alternatively, you can use *input_message_content* to send a message with the specified content instead of the venue.
    ///
    /// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    Venue {
        /// Location latitude in degrees.
        latitude: f32,
        /// Location longitude in degrees.
        longitude: f32,
        /// Location title.
        title: String,
        /// Address of the venue.
        address: String,
        /// Foursquare identifier of the venue if known.
        #[serde(skip_serializing_if = "Option::is_none")]
        foursquare_id: Option<String>,
        /// Foursquare type of the venue, if known.
        /// (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
        #[serde(skip_serializing_if = "Option::is_none")]
        foursquare_type: Option<String>,
        /// Google Places identifier of the venue.
        #[serde(skip_serializing_if = "Option::is_none")]
        google_place_id: Option<String>,
        /// Google Places type of the venue. (See [supported types.](https://developers.google.com/places/web-service/supported_types))
        #[serde(skip_serializing_if = "Option::is_none")]
        google_place_type: Option<String>,
        /// Url of the thumbnail for the result.
        thumb_url: Option<String>,
        /// Thumbnail width.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_width: Option<u32>,
        /// Thumbnail height.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_height: Option<u32>,
        /// Content of the message to be sent instead of the venue.
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    /// A contact with a phone number.
    ///
    /// By default, this contact will be sent by the user.
    /// Alternatively, you can use *input_message_content* to send a message with the specified content instead of the contact.
    ///
    /// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    Contact {
        /// Contact's phone number.
        phone_number: String,
        /// Contact's first name.
        first_name: String,
        /// Contact's last name.
        #[serde(skip_serializing_if = "Option::is_none")]
        last_name: Option<String>,
        /// Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes
        #[serde(skip_serializing_if = "Option::is_none")]
        vcard: Option<String>,
        /// Url of the thumbnail for the result.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_url: Option<String>,
        /// Thumbnail width.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_width: Option<u32>,
        /// Thumbnail height.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_height: Option<u32>,
        /// Content of the message to be sent instead of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    /// A [Game](https://core.telegram.org/bots/api#games).
    Game {
        /// Short name of the game.
        game_short_name: String,
    },
    /// A link to a photo stored on the Telegram servers.
    ///
    /// By default, this photo will be sent by the user with an optional caption.
    /// Alternatively, you can use *input_message_content* to send a message with the specified content instead of the photo.
    ///
    /// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    CachedPhoto {
        /// A valid file identifier of the photo
        photo_file_id: String,
        /// Title for the result
        title: String,
        /// Short description of the result
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
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
        /// Content of the message to be sent instead of the photo
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    /// A link to an animated GIF file stored on the Telegram servers.
    ///
    /// By default, this animated GIF file will be sent by the user with optional caption.
    /// Alternatively, you can use *input_message_content* to send a message with the specified content instead of the animation.
    ///
    /// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    CachedGif {
        /// A valid file identifier for the GIF file.
        gif_file_id: String,
        /// Title for the result.
        title: String,
        /// Caption of the GIF file to be sent, 0-1024 characters after entities parsing.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Mode for parsing entities in the caption.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in the caption,
        /// which can be specified instead of parse_mode
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        /// Content of the message to be sent instead of the GIF animation.
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    /// A link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers.
    ///
    /// By default, this animated MPEG-4 file will be sent by the user with optional caption.
    /// Alternatively, you can use *input_message_content* to send a message with the specified content instead of the animation.
    ///
    /// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    CachedMpeg4Gif {
        /// A valid file identifier for the MP4 file.
        mpeg4_file_id: String,
        /// Title for the result.
        title: String,
        /// Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Mode for parsing entities in the caption.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in the caption,
        /// which can be specified instead of parse_mode.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        /// Content of the message to be sent instead of the video animation.
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    /// A link to a video file stored on the Telegram servers.
    ///
    /// By default, this video file will be sent by the user with an optional caption.
    /// Alternatively, you can use *input_message_content* to send a message with the specified content instead of the video.
    /// > If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube),
    /// > you **must** replace its content using *input_message_content*.
    ///
    /// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    CachedVideo {
        /// A valid file identifier for the video file.
        video_file_id: String,
        /// Title for the result.
        title: String,
        /// Short description of the result.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Caption of the video to be sent, 0-1024 characters after entities parsing.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Mode for parsing entities in the caption.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in the caption,
        /// which can be specified instead of parse_mode.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        /// Content of the message to be sent instead of the video.
        /// This field is **required** if InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    /// A link to an MP3 audio file stored on the Telegram servers.
    ///
    /// By default, this audio file will be sent by the user.
    /// Alternatively, you can use *input_message_content* to send a message with the specified content instead of the audio.
    ///
    /// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    CachedAudio {
        /// A valid file identifier for the audio file.
        audio_file_id: String,
        /// Caption, 0-1024 characters after entities parsing.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Mode for parsing entities in the caption.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in the caption,
        /// which can be specified instead of parse_mode.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        /// Content of the message to be sent instead of the audio.
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    /// A link to a voice message stored on the Telegram servers.
    ///
    /// By default, this voice recording will be sent by the user.
    /// Alternatively, you can use *input_message_content* to send a message with the specified content instead of the the voice message.
    ///
    /// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    CachedVoice {
        /// A valid file identifier for the voice message.
        voice_file_id: String,
        /// Recording title.
        title: String,
        /// Caption, 0-1024 characters after entities parsing.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Mode for parsing entities in the caption.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in the caption,
        /// which can be specified instead of parse_mode.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        /// Content of the message to be sent instead of the voice recording.
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    /// A link to a file stored on the Telegram servers.
    ///
    /// By default, this file will be sent by the user with an optional caption.
    /// Alternatively, you can use *input_message_content* to send a message with the specified content instead of the file.
    /// Currently, only **.PDF** and **.ZIP** files can be sent using this method.
    ///
    /// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    CachedDocument {
        /// A valid file identifier for the file.
        document_file_id: String,
        /// Title for the result.
        title: String,
        /// Short description of the result.
        description: String,
        /// Caption of the document to be sent, 0-1024 characters after entities parsing.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Mode for parsing entities in the caption.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in the caption,
        /// which can be specified instead of parse_mode.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        /// Content of the message to be sent instead of the file.
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
}

impl InlineQueryResultKind {
    /// Creates a inline query result with the given result kind and the given query id.
    pub fn with_id(self, id: impl Into<String>) -> InlineQueryResult {
        use InlineQueryResultKind::*;
        let r#type = match self {
            Article { .. } => "article",
            Photo { .. } | CachedPhoto { .. } => "photo",
            Gif { .. } | CachedGif { .. } => "gif",
            Mpeg4Gif { .. } | CachedMpeg4Gif { .. } => "mpeg4_gif",
            Video { .. } | CachedVideo { .. } => "video",
            Audio { .. } | CachedAudio { .. } => "audio",
            Voice { .. } | CachedVoice { .. } => "voice",
            Document { .. } | CachedDocument { .. } => "document",
            Location { .. } => "location",
            Venue { .. } => "venue",
            Contact { .. } => "contact",
            Game { .. } => "game",
        };
        InlineQueryResult {
            id: id.into(),
            r#type,
            kind: self,
            reply_markup: None,
        }
    }
}

/// The content of a message to be sent as a result of an inline query.
///
/// Telegram clients currently support the following 5 types:
/// - [InputTextMessageContent](https://core.telegram.org/bots/api#inputtextmessagecontent)
/// - [InputLocationMessageContent](https://core.telegram.org/bots/api#inputlocationmessagecontent)
/// - [InputVenueMessageContent](https://core.telegram.org/bots/api#inputvenuemessagecontent)
/// - [InputContactMessageContent](https://core.telegram.org/bots/api#inputcontactmessagecontent)
/// - [InputInvoiceMessageContent](https://core.telegram.org/bots/api#inputinvoicemessagecontent)
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#inputmessagecontent)

#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    /// The [content](https://core.telegram.org/bots/api#inputmessagecontent)
    /// of a text message to be sent as the result of an inline query.
    Text {
        /// Text of the message to be sent, 1-4096 characters.
        message_text: String,
        /// Mode for parsing entities in the message text.
        /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// List of special entities that appear in message text, which can be specified instead of *parse_mode*.
        #[serde(skip_serializing_if = "Option::is_none")]
        entities: Option<Vec<MessageEntity>>,
        /// Disables link previews for links in the sent message.
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_web_page_preview: Option<bool>,
    },
    /// The [content](https://core.telegram.org/bots/api#inputmessagecontent)
    /// of a location message to be sent as the result of an inline query.
    Location {
        /// Latitude of the location in degrees.
        latitude: f32,
        /// Longitude of the location in degrees.
        longitude: f32,
        /// The radius of uncertainty for the location, measured in meters; 0-1500.
        horizontal_accuracy: f32,
        /// Period in seconds for which the location can be updated, should be between 60 and 86400.
        #[serde(skip_serializing_if = "Option::is_none")]
        live_period: Option<u32>,
        /// For live locations, a direction in which the user is moving, in degrees.
        /// Must be between 1 and 360 if specified.
        #[serde(skip_serializing_if = "Option::is_none")]
        heading: Option<u32>,
        /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters.
        /// Must be between 1 and 100000 if specified.
        #[serde(skip_serializing_if = "Option::is_none")]
        proximity_alert_radius: Option<u32>,
    },
    /// The [content](https://core.telegram.org/bots/api#inputmessagecontent)
    /// of a venue message to be sent as the result of an inline query.
    Venue {
        /// Latitude of the venue in degrees.
        latitude: f32,
        /// Longitude of the venue in degrees.
        longitude: f32,
        /// Name of the venue.
        title: String,
        /// Address of the venue.
        address: String,
        /// Foursquare identifier of the venue, if known.
        #[serde(skip_serializing_if = "Option::is_none")]
        foursquare_id: Option<String>,
        /// Foursquare type of the venue, if known.
        /// (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
        #[serde(skip_serializing_if = "Option::is_none")]
        foursquare_type: Option<String>,
        /// Google Places identifier of the venue.
        #[serde(skip_serializing_if = "Option::is_none")]
        google_place_id: Option<String>,
        /// Google Places type of the venue. (See [supported types.](https://developers.google.com/places/web-service/supported_types))
        #[serde(skip_serializing_if = "Option::is_none")]
        google_place_type: Option<String>,
    },
    /// The [content](https://core.telegram.org/bots/api#inputmessagecontent)
    /// of a contact message to be sent as the result of an inline query.
    Contact {
        /// Contact's phone number.
        phone_number: String,
        /// Contact's first name.
        first_name: String,
        /// Contact's last name.
        #[serde(skip_serializing_if = "Option::is_none")]
        last_name: Option<String>,
        /// Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes.
        #[serde(skip_serializing_if = "Option::is_none")]
        vcard: Option<String>,
    },
    /// The [content](https://core.telegram.org/bots/api#inputmessagecontent)
    /// of an invoice message to be sent as the result of an inline query.
    Invoice {
        /// Product name, 1-32 characters.
        title: String,
        /// Product description, 1-255 characters.
        description: String,
        /// Bot-defined invoice payload, 1-128 bytes.
        /// This will not be displayed to the user, use for your internal processes.
        payload: String,
        /// Payment provider token, obtained via [Botfather](https://t.me/botfather).
        provider_token: String,
        /// Three-letter ISO 4217 currency code, see [more on currencies](https://core.telegram.org/bots/payments#supported-currencies).
        currency: String,
        /// Price breakdown, a JSON-serialized list of components.
        /// (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
        prices: Vec<LabeledPrice>,
        /// The maximum accepted amount for tips in the smallest units of the currency (integer, **not** float/double).
        /// For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`.
        /// See the exp parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json),
        /// it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0
        #[serde(skip_serializing_if = "Option::is_none")]
        max_tip_amount: Option<i32>,
        /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, **not** float/double).
        /// At most 4 suggested tip amounts can be specified.
        /// The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed *max_tip_amount*.
        #[serde(skip_serializing_if = "Option::is_none")]
        suggested_tip_amounts: Option<Vec<i32>>,
        /// Unique deep-linking parameter.
        /// If left empty, **forwarded copies** of the sent message will have a *Pay* button, allowing multiple users to pay directly from the forwarded message, using the same invoice.
        /// If non-empty, forwarded copies of the sent message will have a *URL* button with a deep link to the bot (instead of a *Pay* button), with the value used as the start parameter
        #[serde(skip_serializing_if = "Option::is_none")]
        start_parameter: Option<String>,
        /// A JSON-serialized data about the invoice, which will be shared with the payment provider.
        /// A detailed description of required fields should be provided by the payment provider.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_data: Option<String>,
        /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
        /// People like it better when they see what they are paying for.
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_url: Option<String>,
        /// Photo size.
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_size: Option<u32>,
        /// Photo width.
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_width: Option<u32>,
        /// Photo height.
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_height: Option<u32>,
        /// Pass `true`, if you require the user's full name to complete the order.
        #[serde(skip_serializing_if = "Option::is_none")]
        need_name: Option<bool>,
        /// Pass `true`, if you require the user's phone number to complete the order.
        #[serde(skip_serializing_if = "Option::is_none")]
        need_phone_number: Option<bool>,
        /// Pass `true`, if you require the user's email address to complete the order.
        #[serde(skip_serializing_if = "Option::is_none")]
        need_email: Option<bool>,
        /// Pass `true`, if you require the user's shipping address to complete the order
        #[serde(skip_serializing_if = "Option::is_none")]
        need_shipping_address: Option<bool>,
        /// Pass `true`, if user's phone number should be sent to provider
        #[serde(skip_serializing_if = "Option::is_none")]
        send_phone_number_to_provider: Option<bool>,
        /// Pass `true`, if user's email address should be sent to provider
        #[serde(skip_serializing_if = "Option::is_none")]
        send_email_to_provider: Option<bool>,
        /// Pass `true`, if the final price depends on the shipping method
        #[serde(skip_serializing_if = "Option::is_none")]
        is_flexible: Option<bool>,
        /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages).
        /// Users will receive a notification with no sound.
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_notification: Option<bool>,
        /// If the message is a reply, ID of the original message.
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_to_message_id: Option<i64>,
        /// Pass `true`, if the message should be sent even if the specified replied-to message is not found.
        #[serde(skip_serializing_if = "Option::is_none")]
        allow_sending_without_reply: Option<bool>,
        /// A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating).
        /// If empty, one 'Pay `total price`' button will be shown.
        /// If not empty, the first button must be a Pay button.
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
    },
}

/// Sends answers to callback queries sent from inline keyboards.
///
/// The answer will be displayed to the user as a notification at the top of the chat screen or as an alert.
///
/// On success, `true` is returned.
///
/// > Alternatively, the user can be redirected to the specified Game URL.
/// > For this option to work, you must first create a game for your bot via [@Botfather](https://t.me/botfather) and accept the terms.
///
/// Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#answercallbackquery)
#[derive(Clone, Serialize)]
pub struct AnswerCallbackQuery {
    /// Unique identifier for the query to be answered.
    pub callback_query_id: String,
    /// Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// If `true`, an alert will be shown by the client instead of a notification at the top of the chat screen.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<bool>,
    /// URL that will be opened by the user's client.
    // If you have created a [Game](https://core.telegram.org/bots/api#game) and accepted the conditions via [@Botfather](https://t.me/botfather),
    // specify the URL that opens your game.
    /// — note that this will only work if the query comes from a [*callback_game*](https://core.telegram.org/bots/api#inlinekeyboardbutton) button.
    ///
    /// Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The maximum amount of time in seconds that the result of the callback query may be cached client-side.
    /// Telegram apps will support caching starting in version 3.14. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<u32>,
}

impl AnswerCallbackQuery {
    /// Creates a new [`AnswerCallbackQuery`] request that answers to the given callback query.
    pub fn new(query_id: impl Into<String>) -> Self {
        Self {
            callback_query_id: query_id.into(),
            text: None,
            show_alert: None,
            url: None,
            cache_time: None,
        }
    }
    /// Sets text.
    pub fn with_text(self, text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            ..self
        }
    }
    /// Shows alert.
    pub fn show_alert(self) -> Self {
        Self {
            show_alert: Some(true),
            ..self
        }
    }
    /// Sets url.
    pub fn with_url(self, url: impl Into<String>) -> Self {
        Self {
            url: Some(url.into()),
            ..self
        }
    }
    /// Sets cache time.
    pub fn with_cache_time(self, cache_time: u32) -> Self {
        Self {
            cache_time: Some(cache_time),
            ..self
        }
    }
}

impl TelegramMethod for AnswerCallbackQuery {
    type Response = bool;

    fn name() -> &'static str {
        "answerCallbackQuery"
    }
}

impl JsonMethod for AnswerCallbackQuery {}

/// Sends answers to an inline query.
///
/// On success, `true` is returned.
///
/// No more than 50 results per query are allowed.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#answerinlinequery)
#[derive(Clone, Serialize)]
pub struct AnswerInlineQuery {
    /// Unique identifier for the answered query.
    pub inline_query_id: String,
    /// A JSON-serialized array of results for the inline query.
    pub results: Vec<InlineQueryResult>,
    /// The maximum amount of time in seconds that the result of the inline query may be cached on the server.
    /// Defaults to 300.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<u32>,
    /// Pass `true`, if results may be cached on the server side only for the user that sent the query.
    /// By default, results may be returned to any user who sends the same query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,
    /// Pass the offset that a client should send in the next query with the same text to receive more results.
    /// Pass an empty string if there are no more results or if you don't support pagination.
    /// Offset length can't exceed 64 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
    /// If passed, clients will display a button with specified text that switches the user to a private chat with the bot and sends the bot a start message with the parameter switch_pm_parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_text: Option<String>,
    /// [Deep-linking](https://core.telegram.org/bots#deep-linking) parameter for the /start message sent to the bot when user presses the switch button.
    /// 1-64 characters, only `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed.
    ///
    /// *Example:* An inline bot that sends YouTube videos can ask the user to connect the bot to their YouTube account to adapt search results accordingly.
    /// To do this, it displays a 'Connect your YouTube account' button above the results, or even before showing any.
    /// The user presses the button, switches to a private chat with the bot and, in doing so, passes a start parameter that instructs the bot to return an oauth link.
    /// Once done, the bot can offer a [*switch_inline*](https://core.telegram.org/bots/api#inlinekeyboardmarkup) button
    /// so that the user can easily return to the chat where they wanted to use the bot's inline capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_parameter: Option<String>,
}

impl AnswerInlineQuery {
    /// Creates a new [`AnswerInlineQuery`] request that answers to the given query and with given results.
    pub fn new(query_id: impl Into<String>, results: Vec<InlineQueryResult>) -> Self {
        Self {
            inline_query_id: query_id.into(),
            results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            switch_pm_text: None,
            switch_pm_parameter: None,
        }
    }
    /// Sets cache time.
    pub fn with_cache_time(self, cache_time: u32) -> Self {
        Self {
            cache_time: Some(cache_time),
            ..self
        }
    }
    /// Sets the results to be cached on the server side.
    pub fn personal(self) -> Self {
        Self {
            is_personal: Some(true),
            ..self
        }
    }
    /// Sets next offset string.
    pub fn with_next_offset(self, offset: impl Into<String>) -> Self {
        Self {
            next_offset: Some(offset.into()),
            ..self
        }
    }
    /// Sets switch pm text.
    pub fn with_switch_pm_text(self, text: impl Into<String>) -> Self {
        Self {
            switch_pm_text: Some(text.into()),
            ..self
        }
    }
    // Sets switch pm parameter.
    pub fn with_switch_pm_parameter(self, param: impl Into<String>) -> Self {
        Self {
            switch_pm_parameter: Some(param.into()),
            ..self
        }
    }
}

impl TelegramMethod for AnswerInlineQuery {
    type Response = bool;

    fn name() -> &'static str {
        "answerInlineQuery"
    }
}

impl JsonMethod for AnswerInlineQuery {}
