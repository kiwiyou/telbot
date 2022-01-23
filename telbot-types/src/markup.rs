use serde::{Deserialize, Serialize};

use crate::user::User;

/// This object represents a [custom keyboard](https://core.telegram.org/bots#keyboards) with reply options
/// (see [Introduction to bots](https://core.telegram.org/bots#keyboards) for details and examples).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyKeyboardMarkup {
    /// Array of button rows, each represented by an Array of [KeyboardButton](https://core.telegram.org/bots/api#keyboardbutton) objects
    pub keyboard: Vec<Vec<KeyboardButton>>,
    /// Requests clients to resize the keyboard vertically for optimal fit
    // (e.g., make the keyboard smaller if there are just two rows of buttons).
    /// Defaults to false, in which case the custom keyboard is always of the same height as the app's standard keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reisze_keyboard: Option<bool>,
    /// Requests clients to hide the keyboard as soon as it's been used.
    /// The keyboard will still be available, but clients will automatically display the usual letter-keyboard in the chat
    /// – the user can press a special button in the input field to see the custom keyboard again.
    /// Defaults to _false_.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<bool>,
    /// The placeholder to be shown in the input field when the keyboard is active; 1-64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    /// Use this parameter if you want to show the keyboard to specific users only.
    ///
    /// Targets:
    /// 1) users that are @mentioned in the text of the [Message](https://core.telegram.org/bots/api#message) object;
    /// 2) if the bot's message is a reply (has *reply_to_message_id*), sender of the original message.
    ///
    /// Example: A user requests to change the bot's language, bot replies to the request with a keyboard to select the new language.
    /// Other users in the group don't see the keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

/// This object represents one button of the reply keyboard.
/// For simple text buttons *String* can be used instead of this object to specify text of the button.
/// Optional fields *request_contact*, *request_location*, and *request_poll* are mutually exclusive.
///
/// # Note
/// - *request_contact* and *request_location* options will only work in Telegram versions released after 9 April, 2016.
/// Older clients will display *unsupported message*.
/// - *request_poll* option will only work in Telegram versions released after 23 January, 2020.
/// Older clients will display *unsupported message*.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used,
    /// it will be sent as a message when the button is pressed
    pub text: String,
    /// If True, the user's phone number will be sent as a contact when the button is pressed.
    /// Available in private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    request_contact: Option<bool>,
    /// If True, the user's current location will be sent when the button is pressed.
    /// Available in private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    request_location: Option<bool>,
    /// If specified, the user will be asked to create a poll and send it to the bot when the button is pressed.
    /// Available in private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    request_poll: Option<KeyboardButtonPollType>,
}

/// This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButtonPollType {
    /// If *quiz* is passed, the user will be allowed to create only polls in the quiz mode.
    /// If *regular* is passed, only regular polls will be allowed.
    /// Otherwise, the user will be allowed to create a poll of any type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<String>,
}

/// Upon receiving a message with this object, Telegram clients will remove the current custom keyboard and display the default letter-keyboard.
///
/// By default, custom keyboards are displayed until a new keyboard is sent by a bot.
/// An exception is made for one-time keyboards that are hidden immediately after the user presses a button (see ReplyKeyboardMarkup).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyKeyboardRemove {
    /// Requests clients to remove the custom keyboard
    /// (user will not be able to summon this keyboard;
    /// if you want to hide the keyboard from sight but keep it accessible, use *one_time_keyboard* in
    /// [ReplyKeyboardMarkup](https://core.telegram.org/bots/api#replykeyboardmarkup))
    remove_keyboard: bool,
    /// Use this parameter if you want to show the keyboard to specific users only.
    ///
    /// Targets:
    /// 1) users that are @mentioned in the text of the [Message](https://core.telegram.org/bots/api#message) object;
    /// 2) if the bot's message is a reply (has *reply_to_message_id*), sender of the original message.
    ///
    /// Example: A user requests to change the bot's language, bot replies to the request with a keyboard to select the new language.
    /// Other users in the group don't see the keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

/// This object represents an [inline keyboard](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating)
/// that appears right next to the message it belongs to.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of [InlineKeyboardButton](https://core.telegram.org/bots/api#inlinekeyboardbutton) objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    /// Create a new InlineKeyboardMarkup with a row.
    pub fn new_with_row(row: InlineKeyboardRow) -> Self {
        Self {
            inline_keyboard: vec![row.buttons],
        }
    }

    /// Add a row
    pub fn with_row(mut self, row: InlineKeyboardRow) -> Self {
        self.inline_keyboard.push(row.buttons);
        self
    }
}

#[derive(Clone)]
pub struct InlineKeyboardRow {
    pub buttons: Vec<InlineKeyboardButton>,
}

impl InlineKeyboardRow {
    /// Create a new InlineKeyboardRow
    pub fn new_with(button: InlineKeyboardButton) -> Self {
        Self {
            buttons: vec![button],
        }
    }
    /// Create a new InlineKeyboardRow, emplacing a new button
    pub fn new_emplace(text: impl Into<String>, kind: InlineKeyboardButtonKind) -> Self {
        Self {
            buttons: vec![InlineKeyboardButton {
                text: text.into(),
                kind,
            }],
        }
    }
    /// Add a InlineKeyboardButton to the row
    pub fn with(mut self, button: InlineKeyboardButton) -> Self {
        self.buttons.push(button);
        self
    }
    /// Create and add a InlineKeyboardButton to the row
    pub fn emplace(mut self, text: impl Into<String>, kind: InlineKeyboardButtonKind) -> Self {
        self.buttons.push(InlineKeyboardButton {
            text: text.into(),
            kind,
        });
        self
    }
}

/// This object represents one button of an inline keyboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
    /// Label text on the button
    pub text: String,
    /// Button type
    #[serde(flatten)]
    pub kind: InlineKeyboardButtonKind,
}

/// Inline keyboard button type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlineKeyboardButtonKind {
    Url {
        /// HTTP or tg:// url to be opened when button is pressed
        url: String,
    },
    Login {
        /// An HTTP URL used to automatically authorize the user.
        /// Can be used as a replacement for the [Telegram Login Widget](https://core.telegram.org/widgets/login).
        login_url: LoginUrl,
    },
    Callback {
        /// Data to be sent in a [callback query](https://core.telegram.org/bots/api#callbackquery) to the bot when button is pressed, 1-64 bytes
        callback_data: String,
    },
    SwitchInlineQuery {
        /// If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. Can be empty, in which case just the bot's username will be inserted.
        ///
        /// **Note:** This offers an easy way for users to start using your bot in [inline mode](https://core.telegram.org/bots/inline)
        /// when they are currently in a private chat with it.
        /// Especially useful when combined with [*switch_pm…*](https://core.telegram.org/bots/api#answerinlinequery) actions
        /// – in this case the user will be automatically returned to the chat they switched from, skipping the chat selection screen.
        switch_inline_query: String,
    },
    SwitchInlineQueryCurrentChat {
        /// If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field.
        /// Can be empty, in which case only the bot's username will be inserted.
        ///
        /// This offers a quick way for the user to open your bot in inline mode in the same chat
        /// – good for selecting something from multiple options.
        switch_inline_query_current_chat: String,
    },
    CallbackGame {
        /// Description of the game that will be launched when the user presses the button.
        ///
        /// **NOTE:** This type of button **must** always be the first button in the first row.
        callback_game: CallbackGame,
    },
    Pay {
        /// Specify True, to send a Pay button.
        ///
        /// **NOTE:** This type of button **must** always be the first button in the first row.
        pay: bool,
    },
}

impl InlineKeyboardButtonKind {
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Url { url } => Some(url),
            _ => None,
        }
    }

    pub fn login_url(&self) -> Option<&LoginUrl> {
        match self {
            Self::Login { login_url } => Some(login_url),
            _ => None,
        }
    }

    pub fn callback_data(&self) -> Option<&str> {
        match self {
            Self::Callback { callback_data } => Some(&callback_data),
            _ => None,
        }
    }

    pub fn inline_query_prompt(&self) -> Option<&str> {
        match self {
            Self::SwitchInlineQuery {
                switch_inline_query,
            } => Some(switch_inline_query),
            _ => None,
        }
    }

    pub fn inline_query_current_chat_prompt(&self) -> Option<&str> {
        match self {
            Self::SwitchInlineQueryCurrentChat {
                switch_inline_query_current_chat,
            } => Some(switch_inline_query_current_chat),
            _ => None,
        }
    }

    pub fn is_url(&self) -> bool {
        matches!(self, Self::Url { .. })
    }

    pub fn is_login(&self) -> bool {
        matches!(self, Self::Login { .. })
    }

    pub fn is_callback(&self) -> bool {
        matches!(self, Self::Callback { .. })
    }

    pub fn is_switch_inline_query(&self) -> bool {
        matches!(self, Self::SwitchInlineQuery { .. })
    }

    pub fn is_switch_inline_query_current_chat(&self) -> bool {
        matches!(self, Self::SwitchInlineQueryCurrentChat { .. })
    }

    pub fn is_callback_game(&self) -> bool {
        matches!(self, Self::CallbackGame { .. })
    }

    pub fn is_pay(&self) -> bool {
        match self {
            Self::Pay { pay } => *pay,
            _ => false,
        }
    }
}

/// A placeholder, currently holds no information. Use [BotFather](https://t.me/botfather) to set up your game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackGame;

/// This object represents a parameter of the inline keyboard button used to automatically authorize a user.
///
/// Serves as a great replacement for the [Telegram Login Widget](https://core.telegram.org/widgets/login)
/// when the user is coming from Telegram.
/// All the user needs to do is tap/click a button and confirm that they want to log in:
/// ![TITLE](https://core.telegram.org/file/811140015/1734/8VZFkwWXalM.97872/6127fa62d8a0bf2b3c)
/// Telegram apps support buttons as of [version 5.7](https://telegram.org/blog/privacy-discussions-web-bots#meet-seamless-web-bots)
/// > Sample bot: [@discussbot](https://t.me/discussbot)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUrl {
    /// An HTTP URL to be opened with user authorization data added to the query string when the button is pressed.
    ///
    /// If the user refuses to provide authorization data, the original URL without information about the user will be opened.
    /// The data added is the same as described in [Receiving authorization data](https://core.telegram.org/widgets/login#receiving-authorization-data).
    ///
    /// **NOTE:** You **must** always check the hash of the received data to verify the authentication
    /// and the integrity of the data as described in [Checking authorization](https://core.telegram.org/widgets/login#checking-authorization).
    pub url: String,
    /// New text of the button in forwarded messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_text: Option<String>,
    /// Username of a bot, which will be used for user authorization.
    /// See Setting up a bot for more details.
    /// If not specified, the current bot's username will be assumed.
    /// The url's domain must be the same as the domain linked with the bot.
    /// See Linking your domain to the bot for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_username: Option<String>,
    /// Pass True to request the permission for your bot to send messages to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_write_access: Option<bool>,
}

/// Upon receiving a message with this object, Telegram clients will display a reply interface to the user
/// (act as if the user has selected the bot's message and tapped 'Reply').
///
/// This can be extremely useful if you want to create user-friendly step-by-step interfaces without having to sacrifice privacy mode.
///
/// > **Example:** A poll bot for groups runs in privacy mode (only receives commands, replies to its messages and mentions).
/// > There could be two ways to create a new poll:
/// >
/// > - Explain the user how to send a command with parameters (e.g. /newpoll question answer1 answer2).
/// > May be appealing for hardcore users but lacks modern day polish.
/// >
/// > - Guide the user through a step-by-step process.
/// > 'Please send me your question', 'Cool, now let's add the first answer option',
/// > 'Great. Keep adding answer options, then send /done when you're ready'.
/// >
/// > The last option is definitely more attractive.
/// > And if you use [ForceReply](https://core.telegram.org/bots/api#forcereply) in your bot's questions,
/// > it will receive the user's answers even if it only receives replies, commands and mentions
/// > — without any extra work for the user.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceReply {
    /// Shows reply interface to the user, as if they manually selected the bot's message and tapped 'Reply'
    force_reply: bool,
    /// The placeholder to be shown in the input field when the reply is active; 1-64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    /// Use this parameter if you want to show the keyboard to specific users only.
    ///
    /// Targets:
    /// 1) users that are @mentioned in the text of the [Message](https://core.telegram.org/bots/api#message) object;
    /// 2) if the bot's message is a reply (has *reply_to_message_id*), sender of the original message.
    ///
    /// Example: A user requests to change the bot's language, bot replies to the request with a keyboard to select the new language.
    /// Other users in the group don't see the keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Hash)]
pub enum ParseMode {
    MarkdownV2,
    HTML,
    Markdown,
}

impl ParseMode {
    /// Escape text to fit to given parse mode
    pub fn escape(&self, text: impl AsRef<str>) -> String {
        match self {
            Self::MarkdownV2 => Self::escape_markdown_v2(text.as_ref()),
            Self::HTML => Self::escape_html(text.as_ref()),
            Self::Markdown => Self::escape_markdown(text.as_ref()),
        }
    }

    fn escape_markdown_v2(text: &str) -> String {
        const ESCAPE_CHARS: [char; 18] = [
            '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.',
            '!',
        ];
        let mut output = String::with_capacity(text.len());
        let mut block_begin = 0;
        for (index, char) in text.char_indices() {
            if ESCAPE_CHARS.contains(&char) {
                output.push_str(&text[block_begin..index]);
                output.push('\\');
                output.push(char);
                block_begin = index + 1;
            }
        }
        if block_begin < text.len() {
            output.push_str(&text[block_begin..]);
        }
        output
    }

    fn escape_html(text: &str) -> String {
        let mut output = String::with_capacity(text.len());
        let mut block_begin = 0;
        for (index, char) in text.char_indices() {
            let replacement = match char {
                '<' => Some("&lt;"),
                '>' => Some("&gt;"),
                '&' => Some("&amp;"),
                _ => None,
            };
            if let Some(replacement) = replacement {
                output.push_str(&text[block_begin..index]);
                output.push_str(replacement);
                block_begin = index + 1;
            }
        }
        if block_begin < text.len() {
            output.push_str(&text[block_begin..]);
        }
        output
    }

    fn escape_markdown(text: &str) -> String {
        const ESCAPE_CHARS: [char; 4] = ['_', '*', '`', '['];
        let mut output = String::with_capacity(text.len());
        let mut block_begin = 0;
        for (index, char) in text.char_indices() {
            if ESCAPE_CHARS.contains(&char) {
                output.push_str(&text[block_begin..index]);
                output.push('\\');
                output.push(char);
                block_begin = index + 1;
            }
        }
        if block_begin < text.len() {
            output.push_str(&text[block_begin..]);
        }
        output
    }
}

/// This object represents one special entity in a text message.
///
/// For example, hashtags, usernames, URLs, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEntity {
    /// Type of the entity
    #[serde(flatten)]
    pub kind: MessageEntityKind,
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: usize,
    /// Length of the entity in UTF-16 code units
    pub length: usize,
}

/// Type of the message entity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum MessageEntityKind {
    /// `@username`
    Mention,
    /// `#hashtag`
    Hashtag,
    /// `$USD`
    Cashtag,
    /// `/start@jobs_bot`
    BotCommand,
    /// `https://telegram.org`
    Url,
    /// `do-not-reply@telegram.org`
    Email,
    /// `+1-212-555-0123`
    PhoneNumber,
    /// **bold text**
    Bold,
    /// *italic text*
    Italic,
    /// <ins>underlined text</ins>
    Underline,
    /// ~strikethrough text~
    Strikethrough,
    /// `monowidth string`
    Code,
    /// ```monowidth block```
    Pre {
        /// The programming language of the entity text
        language: String,
    },
    /// clickable text URLs
    TextLink {
        /// Url that will be opened after user taps on the text
        url: String,
    },
    /// mention for users [without usernames](https://telegram.org/blog/edit#new-mentions)
    TextMention {
        /// The mentioned user
        user: User,
    },
    /// spoiler message
    Spoiler,
}

impl MessageEntityKind {
    pub fn code_language(&self) -> Option<&str> {
        match self {
            Self::Pre { language } => Some(language),
            _ => None,
        }
    }

    pub fn clickable_url(&self) -> Option<&str> {
        match self {
            Self::TextLink { url } => Some(url),
            _ => None,
        }
    }

    pub fn text_metioned_user(&self) -> Option<&User> {
        match self {
            Self::TextMention { user } => Some(user),
            _ => None,
        }
    }

    pub fn is_mention(&self) -> bool {
        matches!(self, Self::Mention)
    }

    pub fn is_hashtag(&self) -> bool {
        matches!(self, Self::Hashtag)
    }

    pub fn is_cashtag(&self) -> bool {
        matches!(self, Self::Cashtag)
    }

    pub fn is_bot_command(&self) -> bool {
        matches!(self, Self::BotCommand)
    }

    pub fn is_url(&self) -> bool {
        matches!(self, Self::Url)
    }

    pub fn is_email(&self) -> bool {
        matches!(self, Self::Email)
    }

    pub fn is_phone_number(&self) -> bool {
        matches!(self, Self::PhoneNumber)
    }

    pub fn is_bold(&self) -> bool {
        matches!(self, Self::Bold)
    }
    pub fn is_italic(&self) -> bool {
        matches!(self, Self::Italic)
    }

    pub fn is_underline(&self) -> bool {
        matches!(self, Self::Underline)
    }

    pub fn is_strikethrough(&self) -> bool {
        matches!(self, Self::Strikethrough)
    }

    pub fn is_inline_code(&self) -> bool {
        matches!(self, Self::Code)
    }

    pub fn is_code_block(&self) -> bool {
        matches!(self, Self::Pre { .. })
    }

    pub fn is_clickable_link(&self) -> bool {
        matches!(self, Self::TextLink { .. })
    }

    pub fn is_text_mention(&self) -> bool {
        matches!(self, Self::TextMention { .. })
    }
}

/// Reply markups
#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboard(InlineKeyboardMarkup),
    ReplyKeyboard(ReplyKeyboardMarkup),
    RemoveReplyKeyboard(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

impl From<InlineKeyboardMarkup> for ReplyMarkup {
    fn from(markup: InlineKeyboardMarkup) -> Self {
        Self::InlineKeyboard(markup)
    }
}

impl From<ReplyKeyboardMarkup> for ReplyMarkup {
    fn from(markup: ReplyKeyboardMarkup) -> Self {
        Self::ReplyKeyboard(markup)
    }
}

impl From<ReplyKeyboardRemove> for ReplyMarkup {
    fn from(markup: ReplyKeyboardRemove) -> Self {
        Self::RemoveReplyKeyboard(markup)
    }
}

impl From<ForceReply> for ReplyMarkup {
    fn from(markup: ForceReply) -> Self {
        Self::ForceReply(markup)
    }
}
