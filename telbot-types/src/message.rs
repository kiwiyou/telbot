use serde::Deserialize;

use crate::bot::SendMessage;
use crate::chat::Chat;
use crate::file::{Animation, Audio, Document, PhotoSize, Sticker, Video, VideoNote, Voice};
use crate::markup::{InlineKeyboardMarkup, MessageEntity};
use crate::payment::{Invoice, SuccessfulPayment};
use crate::user::User;

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
}
