use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::chat::{Chat, ChatId};
use crate::file::{
    Animation, Audio, Document, InputFile, InputFileVariant, InputMedia, PhotoSize, Video,
    VideoNote, Voice,
};
use crate::markup::{InlineKeyboardMarkup, MessageEntity, ParseMode, ReplyMarkup};
use crate::payment::{Invoice, SuccessfulPayment};
use crate::sticker::Sticker;
use crate::user::User;
use crate::{FileMethod, JsonMethod, TelegramMethod};

/// This object represents a message.
#[derive(Deserialize)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub message_id: i64,
    /// Sender, empty for messages sent to channels
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat.
    /// The channel itself for channel messages.
    /// The supergroup itself for messages from anonymous group administrators.
    /// The linked channel for messages automatically forwarded to the discussion group
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: u64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// For forwarded messages, sender of the original message
    pub forward_from: Option<User>,
    /// For messages forwarded from channels or from anonymous administrators, information about the original sender chat
    pub forward_from_chat: Option<Chat>,
    /// For messages forwarded from channels, identifier of the original message in the channel
    pub forward_from_message_id: Option<i64>,
    /// For messages forwarded from channels, signature of the post author if present
    pub forward_signature: Option<String>,
    /// Sender's name for messages forwarded from users who disallow adding a link to their account in forwarded messages
    pub forward_sender_name: Option<String>,
    /// For forwarded messages, date the original message was sent in Unix time
    pub forward_date: Option<u64>,
    /// For replies, the original message.
    /// Note that the Message object in this field will not contain further reply_to_message fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<u64>,
    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<String>,
    /// Signature of the post author for messages in channels,
    /// or the custom title of an anonymous group administrator
    pub author_signature: Option<String>,
    /// Additional information about the message.
    #[serde(flatten)]
    pub kind: MessageKind,
    /// Inline keyboard attached to the message.
    /// `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Variants of a message.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum MessageKind {
    /// Text message
    Text {
        /// The actual UTF-8 text of the message, 0-4096 characters
        text: String,
        /// Special entities like usernames, URLs, bot commands, etc. that appear in the text
        entities: Option<Vec<MessageEntity>>,
    },
    /// Animation message
    Animation {
        /// Information about the animation.
        /// For backward compatibility, when this field is set, the document field will also be set
        animation: Animation,
        /// Information about the file
        document: Document,
        /// Caption for the animation, 0-1024 characters
        caption: Option<String>,
        /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
        caption_entities: Option<Vec<MessageEntity>>,
    },
    /// Audio message
    Audio {
        /// Information about the file
        audio: Audio,
        /// Caption for the audio, 0-1024 characters
        caption: Option<String>,
        /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
        caption_entities: Option<Vec<MessageEntity>>,
    },
    /// General file message
    Document {
        /// Information about the file
        document: Document,
        /// Caption for the document, 0-1024 characters
        caption: Option<String>,
        /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
        caption_entities: Option<Vec<MessageEntity>>,
    },
    /// Photo message
    Photo {
        /// Available sizes of the photo
        photo: Vec<PhotoSize>,
        /// Caption for the photo, 0-1024 characters
        caption: Option<String>,
        /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
        caption_entities: Option<Vec<MessageEntity>>,
    },
    /// Sticker message
    Sticker {
        /// Information about the sticker
        sticker: Sticker,
    },
    /// Video message
    Video {
        /// Information about the video
        video: Video,
        /// Caption for the video, 0-1024 characters
        caption: Option<String>,
        /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
        caption_entities: Option<Vec<MessageEntity>>,
    },
    /// [Video note](https://telegram.org/blog/video-messages-and-telescope)
    VideoNote {
        /// Information about the video message
        video_note: VideoNote,
    },
    /// Voice message
    Voice {
        /// Information about the file
        voice: Voice,
        /// Caption for the voice, 0-1024 characters
        caption: Option<String>,
        /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
        caption_entities: Option<Vec<MessageEntity>>,
    },
    /// Shared contact
    Contact {
        /// Information about the contact
        contact: Contact,
    },
    Dice {
        dice: Dice,
    },
    Game {
        /// Information about the game.
        /// [More about games »](https://core.telegram.org/bots/api#games)
        game: Game,
    },
    /// Native Poll
    Poll {
        /// Information about the poll
        poll: Poll,
    },
    /// Venue message
    Venue {
        /// Information about the venue.
        /// For backward compatibility, when this field is set, the location field will also be set
        venue: Venue,
        /// Information about the location
        location: Location,
    },
    /// Shared location
    Location {
        /// Information about the location
        location: Location,
    },
    /// New chat members message
    NewChatMembers {
        /// New members that were added to the group or supergroup and information about them
        /// (the bot itself may be one of these members)
        new_chat_members: Vec<User>,
    },
    /// Chat members leave message
    LeftChatMember {
        /// A member was removed from the group, information about them
        /// (this member may be the bot itself)
        left_chat_member: User,
    },
    /// Chat title change message
    NewChatTitle {
        /// A chat title was changed to this value
        new_chat_title: String,
    },
    /// Service message: the chat photo was deleted
    DeleteChatPhoto {
        /// Always true
        delete_chat_photo: bool,
    },
    /// Service message: the group has been created
    GroupChatCreated {
        /// Always true
        group_chat_created: bool,
    },
    /// Service message: the supergroup has been created.
    /// This variant can't be received in a message coming through updates,
    /// because bot can't be a member of a supergroup when it is created.
    /// It can only be found in reply_to_message
    /// if someone replies to a very first message in a directly created supergroup.
    SupergroupChatCreated {
        /// Always true
        supergroup_chat_created: bool,
    },
    /// Service message: the channel has been created.
    /// This variant can't be received in a message coming through updates,
    /// because bot can't be a member of a channel when it is created.
    /// It can only be found in reply_to_message
    /// if someone replies to a very first message in a channel.
    ChannelChatCreated {
        /// Always true
        channel_chat_created: bool,
    },
    /// Service message: auto-delete timer settings changed in the chat
    MessageAutoDeleteTimerChanged {
        message_auto_delete_timer_changed: MessageAutoDeleteTimerChanged,
    },
    /// Group migration message
    GroupMigrated {
        /// The group has been migrated to a supergroup with the specified identifier.
        migrate_to_chat_id: i64,
        /// The supergroup has been migrated from a group with the specified identifier.
        migrate_from_chat_id: i64,
    },
    /// Pinned message
    MessagePinned {
        /// Specified message was pinned.
        /// Note that the Message object in this field will not contain further reply_to_message fields even if it is itself a reply.
        pinned_message: Box<Message>,
    },
    /// Invoice for a [payment](https://core.telegram.org/bots/api#payments)
    Invoice {
        /// Information about the invoice.
        /// [More about payments »](https://core.telegram.org/bots/api#payments)
        invoice: Invoice,
    },
    /// Service message about a successful payment
    SuccessfulPayment {
        /// Information about the payment.
        /// [More about payments »](https://core.telegram.org/bots/api#payments)
        successful_payment: SuccessfulPayment,
    },
    /// Login message.
    Login {
        /// The domain name of the website on which the user has logged in.
        /// [More about Telegram Login »](https://core.telegram.org/widgets/login)
        connected_website: String,
        /// Telegram Passport data
        passport_data: PassportData,
    },
    /// Service message: a user in the chat triggered another user's proximity alert while sharing Live Location
    ProximityAlertTriggered {
        proximity_alert_triggered: ProximityAlertTriggered,
    },
    /// Service message: voice chat scheduled
    VoiceChatScheduled {
        voice_chat_scheduled: VoiceChatScheduled,
    },
    /// Service message: voice chat started
    VoiceChatStarted {
        voice_chat_started: VoiceChatStarted,
    },
    /// Service message: voice chat ended
    VoiceChatEnded {
        voice_chat_ended: VoiceChatEnded,
    },
    /// Service message: new participants invited to a voice chat
    VoiceChatParticipantsInvited {
        voice_chat_participants_invited: VoiceChatParticipantsInvited,
    },
}

/// This object represents a unique message identifier.
#[derive(Deserialize)]
pub struct MessageId {
    /// Unique message identifier
    pub message_id: i64,
}

/// This object represents a point on the map.
#[derive(Deserialize)]
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: f32,
    /// Latitude as defined by sender
    pub latitude: f32,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f32>,
    /// Time relative to the message sending date, during which the location can be updated, in seconds.
    /// For active live locations only.
    pub live_period: Option<i32>,
    /// The direction in which user is moving, in degrees; 1-360.
    /// For active live locations only.
    pub heading: Option<i32>,
    /// Maximum distance for proximity alerts about approaching another chat member, in meters.
    /// For sent live locations only.
    pub proximity_alert_radius: Option<i32>,
}

/// This object represents a phone contact.
#[derive(Deserialize)]
pub struct Contact {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Contact's user identifier in Telegram.
    pub user_id: Option<i64>,
    /// Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard)
    pub vcard: Option<String>,
}

/// This object represents an animated emoji that displays a random value.
#[derive(Deserialize)]
pub struct Dice {
    /// Emoji on which the dice throw animation is based
    pub emoji: String,
    /// Value of the dice, 1-6 for “🎲”, “🎯” and “🎳” base emoji, 1-5 for “🏀” and “⚽” base emoji, 1-64 for “🎰” base emoji
    pub value: i32,
}

#[derive(Deserialize)]
pub struct Game {}

/// This object contains information about one answer option in a poll.
#[derive(Deserialize)]
pub struct PollOption {
    /// Option text, 1-100 characters
    pub text: String,
    /// Number of users that voted for this option
    pub voter_count: u32,
}

/// This object represents an answer of a user in a non-anonymous poll.
#[derive(Deserialize)]
pub struct PollAnswer {
    /// Unique poll identifier
    pub poll_id: String,
    ///  	The user, who changed the answer to the poll
    pub user: User,
    /// 0-based identifiers of answer options, chosen by the user.
    /// May be empty if the user retracted their vote.
    pub option_ids: Vec<u32>,
}

/// This object contains information about a poll.
#[derive(Deserialize)]
pub struct Poll {
    /// Unique poll identifier
    pub id: String,
    /// Poll question, 1-300 characters
    pub question: String,
    /// List of poll options
    pub options: Vec<PollOption>,
    /// Total number of users that voted in the poll
    pub total_voter_count: u32,
    /// True, if the poll is closed
    pub is_closed: bool,
    /// True, if the poll is anonymous
    pub is_anonymous: bool,
    /// Poll type
    #[serde(flatten)]
    pub kind: PollKind,
    /// True, if the poll allows multiple answers
    pub allows_multiple_answers: bool,
    /// Amount of time in seconds the poll will be active after creation
    pub open_period: Option<u32>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed
    pub close_date: Option<u64>,
}

/// Poll type
#[derive(Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum PollKind {
    Regular,
    Quiz {
        /// 0-based identifier of the correct answer option.
        /// Available only for polls in the quiz mode, which are closed,
        /// or was sent (not forwarded) by the bot or to the private chat with the bot.
        correct_option_id: Option<usize>,
        /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll,
        /// 0-200 characters
        explanation: Option<String>,
        /// Special entities like usernames, URLs, bot commands, etc. that appear in the explanation
        explanation_entities: Option<Vec<MessageEntity>>,
    },
}

/// This object represents a venue.
#[derive(Deserialize)]
pub struct Venue {
    /// Venue location. Can't be a live location
    pub location: Location,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue.
    ///
    /// For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See [supported types.](https://developers.google.com/places/web-service/supported_types))
    pub google_place_type: String,
}

/// This object represents a service message about a change in auto-delete timer settings.
#[derive(Deserialize)]
pub struct MessageAutoDeleteTimerChanged {
    /// New auto-delete time for messages in the chat
    pub message_auto_delete_time: u32,
}

#[derive(Deserialize)]
pub struct PassportData {}

/// This object represents the content of a service message,
/// sent whenever a user in the chat triggers a proximity alert set by another user.
#[derive(Deserialize)]
pub struct ProximityAlertTriggered {
    /// User that triggered the alert
    pub traveler: User,
    /// User that set the alert
    pub watcher: User,
    /// The distance between the users
    pub distance: u32,
}

/// This object represents a service message about a voice chat scheduled in the chat.
#[derive(Deserialize)]
pub struct VoiceChatScheduled {
    /// Point in time (Unix timestamp) when the voice chat is supposed to be started by a chat administrator
    pub start_date: u64,
}

/// This object represents a service message about a voice chat started in the chat.
/// Currently holds no information.
#[derive(Deserialize)]
pub struct VoiceChatStarted;

/// This object represents a service message about a voice chat ended in the chat.
#[derive(Deserialize)]
pub struct VoiceChatEnded {
    /// Voice chat duration; in seconds
    pub duration: u32,
}

/// This object represents a service message about new members invited to a voice chat.
#[derive(Deserialize)]
pub struct VoiceChatParticipantsInvited {
    /// New members that were invited to the voice chat
    pub users: Option<Vec<User>>,
}

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

/// Use this method to forward messages of any kind. Service messages can't be forwarded.
/// On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
#[derive(Serialize)]
pub struct ForwardMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Unique identifier for the chat where the original message was sent (in the format `@channelusername`)
    pub from_chat_id: ChatId,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages).
    /// Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Message identifier in the chat specified in *from_chat_id*
    pub message_id: i64,
}

impl ForwardMessage {
    /// Create a new forwardMessage request
    pub fn new(to: impl Into<ChatId>, from: impl Into<ChatId>, message: i64) -> Self {
        Self {
            chat_id: to.into(),
            from_chat_id: from.into(),
            disable_notification: None,
            message_id: message,
        }
    }
    /// Disable notification
    pub fn disable_notification(self) -> Self {
        Self {
            disable_notification: Some(true),
            ..self
        }
    }
}

impl TelegramMethod for ForwardMessage {
    type Response = Message;

    fn name() -> &'static str {
        "forwardMessage"
    }
}

impl JsonMethod for ForwardMessage {}

/// Use this method to copy messages of any kind.
/// Service messages and invoice messages can't be copied.
/// The method is analogous to the method [forwardMessage](https://core.telegram.org/bots/api#forwardmessage), but the copied message doesn't have a link to the original message.
/// Returns the [MessageId](https://core.telegram.org/bots/api#messageid) of the sent message on success.
#[derive(Serialize)]
pub struct CopyMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Unique identifier for the chat where the original message was sent (in the format `@channelusername`)
    pub from_chat_id: ChatId,
    /// Message identifier in the chat specified in *from_chat_id*
    pub message_id: i64,
    /// New caption for media, 0-1024 characters after entities parsing.
    /// If not specified, the original caption is kept
    pub caption: Option<String>,
    /// Mode for parsing entities in the new caption.
    /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<ParseMode>,
    /// List of special entities that appear in the new caption, which can be specified instead of *parse_mode*
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

impl CopyMessage {
    /// Create a new forwardMessage request
    pub fn new(to: impl Into<ChatId>, from: impl Into<ChatId>, message: i64) -> Self {
        Self {
            chat_id: to.into(),
            from_chat_id: from.into(),
            message_id: message,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
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

impl TelegramMethod for CopyMessage {
    type Response = MessageId;

    fn name() -> &'static str {
        "copyMessage"
    }
}

impl JsonMethod for CopyMessage {}

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
            map.insert("video", file);
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

/// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound).
/// On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
/// Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
#[derive(Serialize)]
pub struct SendAnimation {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Animation to send. Pass a file_id as String to send an animation that exists on the Telegram servers (recommended),
    /// pass an HTTP URL as a String for Telegram to get a video from the Internet,
    /// or upload a new video using multipart/form-data.
    /// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
    pub animation: InputFileVariant,
    /// Duration of sent animation in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    /// Animation width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    /// Animation height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
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

impl SendAnimation {
    /// Create a new sendAnimation request
    pub fn new(chat_id: impl Into<ChatId>, animation: impl Into<InputFileVariant>) -> Self {
        Self {
            chat_id: chat_id.into(),
            animation: animation.into(),
            duration: None,
            width: None,
            height: None,
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

impl TelegramMethod for SendAnimation {
    type Response = Message;

    fn name() -> &'static str {
        "sendAnimation"
    }
}

impl FileMethod for SendAnimation {
    fn files(&self) -> Option<HashMap<&str, &InputFile>> {
        let mut map = HashMap::new();
        if let InputFileVariant::File(file) = &self.animation {
            map.insert("animation", file);
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

/// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message.
/// For this to work, your audio must be in an .OGG file encoded with OPUS
/// (other formats may be sent as [Audio](https://core.telegram.org/bots/api#audio) or [Document](https://core.telegram.org/bots/api#document)).
/// On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
/// Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
#[derive(Serialize)]
pub struct SendVoice {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Audio file to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended),
    /// pass an HTTP URL as a String for Telegram to get a video from the Internet,
    /// or upload a new video using multipart/form-data.
    /// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
    pub voice: InputFileVariant,
    /// Duration of the voice message in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
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

impl SendVoice {
    /// Create a new sendVoice request
    pub fn new(chat_id: impl Into<ChatId>, voice: impl Into<InputFileVariant>) -> Self {
        Self {
            chat_id: chat_id.into(),
            voice: voice.into(),
            duration: None,
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

impl TelegramMethod for SendVoice {
    type Response = Message;

    fn name() -> &'static str {
        "sendVoice"
    }
}

impl FileMethod for SendVoice {
    fn files(&self) -> Option<HashMap<&str, &InputFile>> {
        if let InputFileVariant::File(file) = &self.voice {
            let mut map = HashMap::new();
            map.insert("voice", file);
            Some(map)
        } else {
            None
        }
    }
}

/// As of [v.4.0](https://telegram.org/blog/video-messages-and-telescope), Telegram clients support rounded square mp4 videos of up to 1 minute long.
/// Use this method to send video messages.
/// On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
#[derive(Serialize)]
pub struct SendVideoNote {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Video note to send. Pass a file_id as String to send a video note that exists on the Telegram servers (recommended)
    /// or upload a new video using multipart/form-data.
    /// [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
    /// Sending video notes by a URL is currently unsupported
    pub video_note: InputFileVariant,
    /// Duration of sent video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    /// Video width and height, i.e. diameter of the video message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u32>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side.
    /// The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail's width and height should not exceed 320.
    /// Ignored if the file is not uploaded using multipart/form-data.
    /// Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFileVariant>,
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

impl SendVideoNote {
    /// Create a new sendVideoNote request
    pub fn new(chat_id: impl Into<ChatId>, video_note: impl Into<InputFileVariant>) -> Self {
        Self {
            chat_id: chat_id.into(),
            video_note: video_note.into(),
            duration: None,
            length: None,
            thumb: None,
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
    /// Set length
    pub fn with_length(self, length: u32) -> Self {
        Self {
            length: Some(length),
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

impl TelegramMethod for SendVideoNote {
    type Response = Message;

    fn name() -> &'static str {
        "sendVideoNote"
    }
}

impl FileMethod for SendVideoNote {
    fn files(&self) -> Option<HashMap<&str, &InputFile>> {
        let mut map = HashMap::new();
        if let InputFileVariant::File(file) = &self.video_note {
            map.insert("video_note", file);
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
    /// Create a new sendMediaGroup request
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

impl Message {
    pub fn text(&self) -> Option<&str> {
        match &self.kind {
            MessageKind::Text { text, .. } => Some(text),
            _ => None,
        }
    }

    pub fn reply_text(&self, text: impl Into<String>) -> SendMessage {
        SendMessage::new(self.chat.id, text).reply_to(self.message_id)
    }

    pub fn forward_to(&self, chat_id: impl Into<ChatId>) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id, self.message_id)
    }

    pub fn copy_to(&self, chat_id: impl Into<ChatId>) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id, self.message_id)
    }
}

/// Use this method to send point on the map.
/// On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
#[derive(Serialize)]
pub struct SendLocation {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Latitude of the location
    pub latitude: f32,
    /// Longitude of the location
    pub longitude: f32,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: f32,
    /// Period in seconds for which the location can be updated
    /// (see [Live Locations](https://telegram.org/blog/live-locations)), should be between 60 and 86400.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<u32>,
    /// For live locations, a direction in which the user is moving, in degrees.
    /// Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u32>,
    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters.
    /// Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u32>,
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

impl SendLocation {
    /// Create a new sendLocation request
    pub fn new(
        chat_id: impl Into<ChatId>,
        latitude: f32,
        longitude: f32,
        horizontal_accuracy: f32,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            latitude,
            longitude,
            horizontal_accuracy,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    /// Set live period
    pub fn with_live_period(self, live_period: u32) -> Self {
        Self {
            live_period: Some(live_period),
            ..self
        }
    }
    /// Set heading
    pub fn with_heading(self, direction: u32) -> Self {
        Self {
            heading: Some(direction),
            ..self
        }
    }
    /// Set proximity alert radius
    pub fn proximity_alert_within(self, radius: u32) -> Self {
        Self {
            proximity_alert_radius: Some(radius),
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

impl TelegramMethod for SendLocation {
    type Response = Message;

    fn name() -> &'static str {
        "sendLocation"
    }
}

impl JsonMethod for SendLocation {}

/// Use this method to edit live location messages.
/// A location can be edited until its *live_period* expires
/// or editing is explicitly disabled by a call to [stopMessageLiveLocation](https://core.telegram.org/bots/api#stopmessagelivelocation).
/// On success, the edited [Message](https://core.telegram.org/bots/api#message) is returned.
#[derive(Serialize)]
pub struct EditMessageLiveLocation {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Identifier of the message to edit
    pub message_id: i64,
    /// Latitude of new location
    pub latitude: f32,
    /// Longitude of new location
    pub longitude: f32,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f32>,
    /// For live locations, a direction in which the user is moving, in degrees.
    /// Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u32>,
    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters.
    /// Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u32>,
    /// A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageLiveLocation {
    /// Create a new editMessageLiveLocation request
    pub fn new(chat_id: impl Into<ChatId>, message_id: i64, latitude: f32, longitude: f32) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id,
            latitude,
            longitude,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
        }
    }
    /// Set horizontal accuracy
    pub fn with_horizontal_accuracy(self, accuracy: f32) -> Self {
        Self {
            horizontal_accuracy: Some(accuracy),
            ..self
        }
    }
    /// Set heading
    pub fn with_heading(self, direction: u32) -> Self {
        Self {
            heading: Some(direction),
            ..self
        }
    }
    /// Set proximity alert radius
    pub fn proximity_alert_within(self, radius: u32) -> Self {
        Self {
            proximity_alert_radius: Some(radius),
            ..self
        }
    }
    /// Set reply markup
    pub fn with_reply_markup(self, markup: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(markup.into()),
            ..self
        }
    }
}

impl TelegramMethod for EditMessageLiveLocation {
    type Response = Message;

    fn name() -> &'static str {
        "editMessageLiveLocation"
    }
}

impl JsonMethod for EditMessageLiveLocation {}

/// Use this method to edit live location messages.
/// A location can be edited until its *live_period* expires
/// or editing is explicitly disabled by a call to [stopMessageLiveLocation](https://core.telegram.org/bots/api#stopmessagelivelocation).
/// On success, _True_ is returned.
#[derive(Serialize)]
pub struct EditInlineMessageLiveLocation {
    /// Identifier of the inline message
    pub inline_message_id: String,
    /// Latitude of new location
    pub latitude: f32,
    /// Longitude of new location
    pub longitude: f32,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f32>,
    /// For live locations, a direction in which the user is moving, in degrees.
    /// Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u32>,
    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters.
    /// Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u32>,
    /// A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditInlineMessageLiveLocation {
    /// Create a new editMessageLiveLocation request
    pub fn new(inline_message_id: impl Into<String>, latitude: f32, longitude: f32) -> Self {
        Self {
            inline_message_id: inline_message_id.into(),
            latitude,
            longitude,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
        }
    }
    /// Set horizontal accuracy
    pub fn with_horizontal_accuracy(self, accuracy: f32) -> Self {
        Self {
            horizontal_accuracy: Some(accuracy),
            ..self
        }
    }
    /// Set heading
    pub fn with_heading(self, direction: u32) -> Self {
        Self {
            heading: Some(direction),
            ..self
        }
    }
    /// Set proximity alert radius
    pub fn proximity_alert_within(self, radius: u32) -> Self {
        Self {
            proximity_alert_radius: Some(radius),
            ..self
        }
    }
    /// Set reply markup
    pub fn with_reply_markup(self, markup: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(markup.into()),
            ..self
        }
    }
}

impl TelegramMethod for EditInlineMessageLiveLocation {
    type Response = bool;

    fn name() -> &'static str {
        "editMessageLiveLocation"
    }
}

impl JsonMethod for EditInlineMessageLiveLocation {}

/// Use this method to stop updating a live location message before live_period expires.
/// On success, the edited [Message](https://core.telegram.org/bots/api#message) is returned.
#[derive(Serialize)]
pub struct StopMessageLiveLocation {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Identifier of the message to edit
    pub message_id: i64,
    /// A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl StopMessageLiveLocation {
    /// Create a new stopMessageLiveLocation request from chat message id
    pub fn from_chat(chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id,
            reply_markup: None,
        }
    }
    /// Set reply markup
    pub fn with_reply_markup(self, markup: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(markup.into()),
            ..self
        }
    }
}

impl TelegramMethod for StopMessageLiveLocation {
    type Response = Message;

    fn name() -> &'static str {
        "stopMessageLiveLocation"
    }
}

impl JsonMethod for StopMessageLiveLocation {}

/// Use this method to stop updating a live location message before live_period expires.
/// On success, _True_ is returned.
#[derive(Serialize)]
pub struct StopInlineMessageLiveLocation {
    /// Identifier of the inline message
    pub inline_message_id: String,
    /// A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl StopInlineMessageLiveLocation {
    /// Create a new stopMessageLiveLocation request
    pub fn new(inline_message_id: impl Into<String>) -> Self {
        Self {
            inline_message_id: inline_message_id.into(),
            reply_markup: None,
        }
    }
    /// Set reply markup
    pub fn with_reply_markup(self, markup: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(markup.into()),
            ..self
        }
    }
}

impl TelegramMethod for StopInlineMessageLiveLocation {
    type Response = bool;

    fn name() -> &'static str {
        "stopMessageLiveLocation"
    }
}

impl JsonMethod for StopInlineMessageLiveLocation {}

/// Use this method to send information about a venue.
/// On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
#[derive(Serialize)]
pub struct SendVenue {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Latitude of the venue
    pub latitude: f32,
    /// Longitude of the venue
    pub longitude: f32,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue, if known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known.
    /// (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See [supported types.](https://developers.google.com/places/web-service/supported_types))
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
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

impl SendVenue {
    /// Create a new sendVenue request
    pub fn new(
        chat_id: impl Into<ChatId>,
        latitude: f32,
        longitude: f32,
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    /// Set foursquare id and type
    pub fn with_foursqaure(self, id: impl Into<String>, kind: Option<String>) -> Self {
        Self {
            foursquare_id: Some(id.into()),
            foursquare_type: kind,
            ..self
        }
    }
    /// Set google place id and type
    pub fn with_google_place(self, id: impl Into<String>, kind: Option<String>) -> Self {
        Self {
            google_place_id: Some(id.into()),
            google_place_type: kind,
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

impl TelegramMethod for SendVenue {
    type Response = Message;

    fn name() -> &'static str {
        "sendVenue"
    }
}

impl JsonMethod for SendVenue {}

/// Use this method to send text messages. On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
#[derive(Serialize)]
pub struct SendContact {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
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

impl SendContact {
    /// Create a new sendContact request
    pub fn new(
        chat_id: impl Into<ChatId>,
        phone_number: impl Into<String>,
        first_name: impl Into<String>,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    /// Set last name
    pub fn with_last_name(self, last_name: impl Into<String>) -> Self {
        Self {
            last_name: Some(last_name.into()),
            ..self
        }
    }
    /// Set vcard
    pub fn with_vcard(self, vcard: impl Into<String>) -> Self {
        Self {
            vcard: Some(vcard.into()),
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

impl TelegramMethod for SendContact {
    type Response = Message;

    fn name() -> &'static str {
        "sendContact"
    }
}

impl JsonMethod for SendContact {}

/// Use this method to send a native poll. On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
#[derive(Serialize)]
pub struct SendPoll {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Poll question, 1-300 characters
    pub question: String,
    /// A JSON-serialized list of answer options, 2-10 strings 1-100 characters each
    pub options: Vec<String>,
    /// True, if the poll needs to be anonymous, defaults to *True*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// Poll type, “quiz” or “regular”, defaults to “regular”
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub kind: Option<String>,
    /// True, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to *False*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_multiple_answers: Option<bool>,
    /// 0-based identifier of the correct answer option, required for polls in quiz mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<u32>,
    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll,
    /// 0-200 characters with at most 2 line feeds after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Mode for parsing entities in the explanation.
    /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<ParseMode>,
    /// List of special entities that appear in the poll explanation, which can be specified instead of *parse_mode*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,
    /// Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with *close_date*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<u32>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed.
    /// Must be at least 5 and no more than 600 seconds in the future.
    /// Can't be used together with open_period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<u64>,
    /// Pass *True*, if the poll needs to be immediately closed.
    /// This can be useful for poll preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<bool>,
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

impl SendPoll {
    /// Create a new sendPoll request to send a regular poll
    pub fn new_regular(
        chat_id: impl Into<ChatId>,
        question: impl Into<String>,
        options: Vec<String>,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            question: question.into(),
            options,
            is_anonymous: None,
            kind: Some("quiz".into()),
            allows_multiple_answers: None,
            correct_option_id: None,
            explanation: None,
            explanation_parse_mode: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
            is_closed: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    /// Create a new sendPoll request to send a quiz
    pub fn new_quiz(
        chat_id: impl Into<ChatId>,
        question: impl Into<String>,
        options: Vec<String>,
        correct_option_id: u32,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            question: question.into(),
            options,
            is_anonymous: None,
            kind: Some("quiz".into()),
            allows_multiple_answers: None,
            correct_option_id: Some(correct_option_id),
            explanation: None,
            explanation_parse_mode: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
            is_closed: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    /// Set the poll as anonymous
    pub fn anonymous(self) -> Self {
        Self {
            is_anonymous: Some(true),
            ..self
        }
    }
    /// Allow multiple answers
    pub fn allow_multiple_answers(self) -> Self {
        Self {
            allows_multiple_answers: Some(true),
            ..self
        }
    }
    /// Set explanation
    pub fn with_explanation(self, explanation: impl Into<String>) -> Self {
        Self {
            explanation: Some(explanation.into()),
            ..self
        }
    }
    /// Set explanation parse mode
    pub fn with_parse_mode(self, parse_mode: ParseMode) -> Self {
        Self {
            explanation_parse_mode: Some(parse_mode),
            ..self
        }
    }
    /// Set explanation entities
    pub fn with_entities(self, entities: Vec<MessageEntity>) -> Self {
        Self {
            explanation_entities: Some(entities),
            ..self
        }
    }
    /// Add explanation entity
    pub fn with_entity(mut self, entity: MessageEntity) -> Self {
        let entities = self
            .explanation_entities
            .get_or_insert_with(Default::default);
        entities.push(entity);
        self
    }
    /// Set open period. This sets `close_date` to `None`
    pub fn with_open_period(self, period: u32) -> Self {
        Self {
            open_period: Some(period),
            close_date: None,
            ..self
        }
    }
    /// Set close date. This sets `open_period` to `None`
    pub fn with_close_date(self, close_date: u64) -> Self {
        Self {
            close_date: Some(close_date),
            open_period: None,
            ..self
        }
    }
    /// Set the poll as closed
    pub fn closed(self) -> Self {
        Self {
            is_closed: Some(true),
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

impl TelegramMethod for SendPoll {
    type Response = Message;

    fn name() -> &'static str {
        "sendPoll"
    }
}

impl JsonMethod for SendPoll {}

/// Use this method to send an animated emoji that will display a random value.
/// On success, the sent [Message](https://core.telegram.org/bots/api#message) is returned.
#[derive(Serialize)]
pub struct SendDice {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Emoji on which the dice throw animation is based.
    /// Currently, must be one of “🎲”, “🎯”, “🏀”, “⚽”, “🎳”, or “🎰”.
    /// Dice can have values 1-6 for “🎲”, “🎯” and “🎳”, values 1-5 for “🏀” and “⚽”, and values 1-64 for “🎰”. Defaults to “🎲”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
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

impl SendDice {
    /// Create a new sendDice request.
    pub fn new(chat_id: impl Into<ChatId>) -> Self {
        Self {
            chat_id: chat_id.into(),
            emoji: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
    /// Set emoji
    pub fn with_emoji(self, emoji: impl Into<String>) -> Self {
        Self {
            emoji: Some(emoji.into()),
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

impl TelegramMethod for SendDice {
    type Response = Message;

    fn name() -> &'static str {
        "sendDice"
    }
}

impl JsonMethod for SendDice {}

/// Chat action type
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatActionKind {
    Typing,
    UploadPhoto,
    RecordVideo,
    UploadVideo,
    RecordVoice,
    UploadVoice,
    UplaodDocument,
    FindLocation,
    RecordVideoNote,
    UploadVideoNote,
}

/// Use this method when you need to tell the user that something is happening on the bot's side.
/// The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status).
/// Returns True on success.
///
/// > Example: The [ImageBot](https://t.me/imagebot) needs some time to process a request and upload the image.
/// > Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use sendChatAction with action = upload_photo.
/// > The user will see a “sending photo” status for the bot.
///
/// It is recommended to use this method only when a response from the bot will take a noticeable amount of time to arrive.
#[derive(Serialize)]
pub struct SendChatAction {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Type of action to broadcast.
    pub action: ChatActionKind,
}

impl SendChatAction {
    /// Create a new sendChatAction request.
    pub fn new(chat_id: impl Into<ChatId>, action: ChatActionKind) -> Self {
        Self {
            chat_id: chat_id.into(),
            action,
        }
    }
}

impl TelegramMethod for SendChatAction {
    type Response = Message;

    fn name() -> &'static str {
        "sendChatAction"
    }
}

impl JsonMethod for SendChatAction {}

/// Use this method to edit text and [game](https://core.telegram.org/bots/api#games) messages.
/// On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api#message) is returned,
/// otherwise _True_ is returned.
#[derive(Serialize)]
pub struct EditMessageText {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Identifier of the message to edit
    pub message_id: i64,
    /// New text of the message, 1-4096 characters after entities parsing
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
    /// A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageText {
    /// Create a new editMessageText request
    pub fn new(chat_id: impl Into<ChatId>, message_id: i64, text: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id,
            text: text.into(),
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
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
    /// Set reply markup
    pub fn with_reply_markup(self, markup: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(markup.into()),
            ..self
        }
    }
}

impl TelegramMethod for EditMessageText {
    type Response = Message;

    fn name() -> &'static str {
        "editMessageText"
    }
}

impl JsonMethod for EditMessageText {}

/// Use this method to edit text and [game](https://core.telegram.org/bots/api#games) messages.
/// On success, _True_ is returned.
#[derive(Serialize)]
pub struct EditInlineMessageText {
    /// Identifier of the inline message
    pub inline_message_id: String,
    /// New text of the message, 1-4096 characters after entities parsing
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
    /// A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditInlineMessageText {
    /// Create a new editMessageText request
    pub fn new(inline_message_id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            inline_message_id: inline_message_id.into(),
            text: text.into(),
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
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
    /// Set reply markup
    pub fn with_reply_markup(self, markup: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(markup.into()),
            ..self
        }
    }
}

impl TelegramMethod for EditInlineMessageText {
    type Response = bool;

    fn name() -> &'static str {
        "editMessageText"
    }
}

impl JsonMethod for EditInlineMessageText {}

/// Use this method to edit captions of messages. On success, the edited Message is returned.
#[derive(Serialize)]
pub struct EditMessageCaption {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Identifier of the message to edit
    pub message_id: i64,
    /// New caption of the message, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Mode for parsing entities in the message text.
    /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Disables link previews for links in the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    /// A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageCaption {
    /// Create a new editMessageCaption request with no caption
    pub fn new_empty(chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }
    /// Create a new editMessageCaption request with caption
    pub fn new(chat_id: impl Into<ChatId>, message_id: i64, caption: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id,
            caption: Some(caption.into()),
            parse_mode: None,
            caption_entities: None,
            disable_web_page_preview: None,
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
    /// Disable web preview
    pub fn disable_web_page_preview(self) -> Self {
        Self {
            disable_web_page_preview: Some(true),
            ..self
        }
    }
    /// Set reply markup
    pub fn with_reply_markup(self, markup: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(markup.into()),
            ..self
        }
    }
}

impl TelegramMethod for EditMessageCaption {
    type Response = Message;

    fn name() -> &'static str {
        "editMessageCaption"
    }
}

impl JsonMethod for EditMessageCaption {}

/// Use this method to edit captions of messages. On success, the edited Message is returned.
#[derive(Serialize)]
pub struct EditInlineMessageCaption {
    /// Identifier of the inline message
    pub inline_message_id: String,
    /// New caption of the message, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Mode for parsing entities in the message text.
    /// See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Disables link previews for links in the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    /// A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditInlineMessageCaption {
    /// Create a new editMessageCaption request with no caption
    pub fn new_empty(inline_message_id: impl Into<String>) -> Self {
        Self {
            inline_message_id: inline_message_id.into(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }
    /// Create a new editMessageCaption request with caption
    pub fn new(inline_message_id: impl Into<String>, caption: impl Into<String>) -> Self {
        Self {
            inline_message_id: inline_message_id.into(),
            caption: Some(caption.into()),
            parse_mode: None,
            caption_entities: None,
            disable_web_page_preview: None,
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
    /// Disable web preview
    pub fn disable_web_page_preview(self) -> Self {
        Self {
            disable_web_page_preview: Some(true),
            ..self
        }
    }
    /// Set reply markup
    pub fn with_reply_markup(self, markup: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(markup.into()),
            ..self
        }
    }
}

impl TelegramMethod for EditInlineMessageCaption {
    type Response = bool;

    fn name() -> &'static str {
        "editMessageCaption"
    }
}

impl JsonMethod for EditInlineMessageCaption {}

/// Use this method to edit animation, audio, document, photo, or video messages.
/// If a message is part of a message album, then it can be edited only to an audio for audio albums,
/// only to a document for document albums and to a photo or a video otherwise.
/// When an inline message is edited, a new file can't be uploaded;
/// use a previously uploaded file via its file_id or specify a URL.
/// On success, the edited [Message](https://core.telegram.org/bots/api#message) is returned.
#[derive(Serialize)]
pub struct EditMessageMedia {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Identifier of the message to edit
    pub message_id: i64,
    /// A JSON-serialized object for a new media content of the message
    pub media: InputMedia,
    /// A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageMedia {
    /// Create a new editMessageMedia request
    pub fn new(chat_id: impl Into<ChatId>, message_id: i64, media: impl Into<InputMedia>) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id,
            media: media.into(),
            reply_markup: None,
        }
    }
    /// Set reply markup
    pub fn with_reply_markup(self, markup: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(markup.into()),
            ..self
        }
    }
}

impl TelegramMethod for EditMessageMedia {
    type Response = Message;

    fn name() -> &'static str {
        "editMessageMedia"
    }
}

impl JsonMethod for EditMessageMedia {}

/// Use this method to edit animation, audio, document, photo, or video messages.
/// If a message is part of a message album, then it can be edited only to an audio for audio albums,
/// only to a document for document albums and to a photo or a video otherwise.
/// When an inline message is edited, a new file can't be uploaded;
/// use a previously uploaded file via its file_id or specify a URL.
/// On success, _True_ is returned.
#[derive(Serialize)]
pub struct EditInlineMessageMedia {
    /// Identifier of the inline message
    pub inline_message_id: String,
    /// A JSON-serialized object for a new media content of the message
    pub media: InputMedia,
    /// A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditInlineMessageMedia {
    /// Create a new editMessageMedia request
    pub fn new(inline_message_id: impl Into<String>, media: impl Into<InputMedia>) -> Self {
        Self {
            inline_message_id: inline_message_id.into(),
            media: media.into(),
            reply_markup: None,
        }
    }
    /// Set reply markup
    pub fn with_reply_markup(self, markup: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(markup.into()),
            ..self
        }
    }
}

impl TelegramMethod for EditInlineMessageMedia {
    type Response = bool;

    fn name() -> &'static str {
        "editMessageMedia"
    }
}

impl JsonMethod for EditInlineMessageMedia {}

/// Use this method to edit only the reply markup of messages. On success, the edited Message is returned.
#[derive(Serialize)]
pub struct EditMessageReplyMarkup {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Identifier of the message to edit
    pub message_id: i64,
    /// A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageReplyMarkup {
    /// Create a new editMessageReplyMarkup request with no reply markup
    pub fn new_empty(chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id,
            reply_markup: None,
        }
    }
    /// Create a new editMessageReplyMarkup with reply markup
    pub fn new(
        chat_id: impl Into<ChatId>,
        message_id: i64,
        reply_markup: impl Into<InlineKeyboardMarkup>,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id,
            reply_markup: Some(reply_markup.into()),
        }
    }
}

impl TelegramMethod for EditMessageReplyMarkup {
    type Response = Message;

    fn name() -> &'static str {
        "editMessageReplyMarkup"
    }
}

impl JsonMethod for EditMessageReplyMarkup {}

/// Use this method to edit only the reply markup of messages. On success, _True_ is returned.
#[derive(Serialize)]
pub struct EditInlineMessageReplyMarkup {
    /// Identifier of the inline message
    pub inline_message_id: String,
    /// A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditInlineMessageReplyMarkup {
    /// Create a new editMessageReplyMarkup request with no reply markup
    pub fn new_empty(inline_message_id: impl Into<String>) -> Self {
        Self {
            inline_message_id: inline_message_id.into(),
            reply_markup: None,
        }
    }
    /// Create a new editMessageReplyMarkup with reply markup
    pub fn new(
        inline_message_id: impl Into<String>,
        reply_markup: impl Into<InlineKeyboardMarkup>,
    ) -> Self {
        Self {
            inline_message_id: inline_message_id.into(),
            reply_markup: Some(reply_markup.into()),
        }
    }
}

impl TelegramMethod for EditInlineMessageReplyMarkup {
    type Response = bool;

    fn name() -> &'static str {
        "editMessageReplyMarkup"
    }
}

impl JsonMethod for EditInlineMessageReplyMarkup {}

/// Use this method to stop a poll which was sent by the bot.
/// On success, the stopped [Poll](https://core.telegram.org/bots/api#poll) is returned.
#[derive(Serialize)]
pub struct StopPoll {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Identifier of the original message with the poll
    pub message_id: i64,
    /// A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl StopPoll {
    /// Create a new stopPoll request
    pub fn new(chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id,
            reply_markup: None,
        }
    }
    /// Set reply markup
    pub fn with_reply_markup(self, markup: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(markup.into()),
            ..self
        }
    }
}

impl TelegramMethod for StopPoll {
    type Response = Poll;

    fn name() -> &'static str {
        "stopPoll"
    }
}

impl JsonMethod for StopPoll {}

/// Use this method to delete a message, including service messages, with the following limitations:
/// - A message can only be deleted if it was sent less than 48 hours ago.
/// - A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.
/// - Bots can delete outgoing messages in private chats, groups, and supergroups.
/// - Bots can delete incoming messages in private chats.
/// - Bots granted *can_post_messages* permissions can delete outgoing messages in channels.
/// - If the bot is an administrator of a group, it can delete any message there.
/// - If the bot has *can_delete_messages* permission in a supergroup or a channel, it can delete any message there.
/// Returns _True_ on success.
#[derive(Serialize)]
pub struct DeleteMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Identifier of the message to delete
    pub message_id: i64,
}

impl DeleteMessage {
    /// Create a new deleteMessage request
    pub fn new(chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id,
        }
    }
}

impl TelegramMethod for DeleteMessage {
    type Response = bool;

    fn name() -> &'static str {
        "deleteMessage"
    }
}

impl JsonMethod for DeleteMessage {}
