use serde::{Deserialize, Serialize};

use crate::chat::ChatMemberUpdated;
use crate::message::{Message, Poll, PollAnswer};
use crate::payment::{PreCheckoutQuery, ShippingQuery};
use crate::query::{CallbackQuery, ChosenInlineResult, InlineQuery};
use crate::{JsonMethod, TelegramMethod};

/// An incoming update.
///
/// At most **one** of the optional parameters can be present in any given update.
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#update)
#[derive(Debug, Deserialize)]
pub struct Update {
    /// The update's unique identifier.
    /// Update identifiers start from a certain positive number and increase sequentially.
    /// This ID becomes especially handy if you're using Webhooks,
    /// since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order.
    /// If there are no new updates for at least a week,
    /// then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: u32,
    #[serde(flatten)]
    /// Update type.
    pub kind: UpdateKind,
}

/// Type of update.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum UpdateKind {
    /// New incoming message of any kind — text, photo, sticker, etc.
    Message { message: Message },
    /// New version of a message that is known to the bot and was edited.
    EditedMessage { edited_message: Message },
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    ChannelPost { channel_post: Message },
    /// New version of a channel post that is known to the bot and was edited.
    EditedChannelPost { edited_channel_post: Message },
    /// New incoming [inline](https://core.telegram.org/bots/api#inline-mode) query.
    InlineQuery { inline_query: InlineQuery },
    /// The result of an [inline](https://core.telegram.org/bots/api#inline-mode)
    /// query that was chosen by a user and sent to their chat partner.
    /// Please see Telegram's documentation on the [feedback collecting](https://core.telegram.org/bots/inline#collecting-feedback) for details
    /// on how to enable these updates for your bot.
    ChosenInlineResult {
        chosen_inline_result: ChosenInlineResult,
    },
    /// New incoming callback query.
    CallbackQuery { callback_query: CallbackQuery },
    /// New incoming shipping query. Only for invoices with flexible price.
    ShippingQuery { shipping_query: ShippingQuery },
    /// New incoming pre-checkout query. Contains full information about checkout.
    PreCheckoutQuery {
        pre_checkout_query: PreCheckoutQuery,
    },
    /// New poll state.
    /// Bots receive only updates about stopped polls and polls, which are sent by the bot.
    Poll { poll: Poll },
    /// A user changed their answer in a non-anonymous poll.
    /// Bots receive new votes only in polls that were sent by the bot itself.
    PollAnswer { poll_answer: PollAnswer },
    /// The bot's chat member status was updated in a chat. For private chats,
    /// this update is received only when the bot is blocked or unblocked by the user.
    MyChatMemberUpdated { my_chat_member: ChatMemberUpdated },
    /// A chat member's status was updated in a chat.
    /// The bot must be an administrator in the chat and must explicitly specify “chat_member”
    /// in the list of *allowed_updates* to receive these updates.
    ChatMemberUpdated { chat_member: ChatMemberUpdated },
}

impl UpdateKind {
    /// Gets the message associated with this update, if any.
    pub fn message(&self) -> Option<&Message> {
        match self {
            Self::Message { message } => Some(message),
            _ => None,
        }
    }

    /// Gets the edited message associated with this update, if any.
    pub fn edited_message(&self) -> Option<&Message> {
        match self {
            Self::EditedMessage { edited_message } => Some(edited_message),
            _ => None,
        }
    }

    /// Gets the channel post associated with this update, if any.
    pub fn channel_post(&self) -> Option<&Message> {
        match self {
            Self::ChannelPost { channel_post } => Some(channel_post),
            _ => None,
        }
    }

    /// Gets the edited channel post associated with this update, if any.
    pub fn edited_channel_post(&self) -> Option<&Message> {
        match self {
            Self::EditedChannelPost {
                edited_channel_post,
            } => Some(edited_channel_post),
            _ => None,
        }
    }

    /// Gets the inline query associated with this update, if any.
    pub fn inline_query(&self) -> Option<&InlineQuery> {
        match self {
            Self::InlineQuery { inline_query } => Some(inline_query),
            _ => None,
        }
    }

    /// Gets the chosen inline result associated with this update, if any.
    pub fn chosen_inline_result(&self) -> Option<&ChosenInlineResult> {
        match self {
            Self::ChosenInlineResult {
                chosen_inline_result,
            } => Some(chosen_inline_result),
            _ => None,
        }
    }

    /// Gets the callback query associated with this update, if any.
    pub fn callback_query(&self) -> Option<&CallbackQuery> {
        match self {
            Self::CallbackQuery { callback_query } => Some(callback_query),
            _ => None,
        }
    }

    /// Gets the shipping query associated with this update, if any.
    pub fn shipping_query(&self) -> Option<&ShippingQuery> {
        match self {
            Self::ShippingQuery { shipping_query } => Some(shipping_query),
            _ => None,
        }
    }

    /// Gets the pre checkout query associated with this update, if any.
    pub fn pre_checkout_query(&self) -> Option<&PreCheckoutQuery> {
        match self {
            Self::PreCheckoutQuery { pre_checkout_query } => Some(pre_checkout_query),
            _ => None,
        }
    }

    /// Gets the poll associated with this update, if any.
    pub fn poll(&self) -> Option<&Poll> {
        match self {
            Self::Poll { poll } => Some(poll),
            _ => None,
        }
    }

    /// Gets the poll answer associated with this update, if any.
    pub fn poll_answer(&self) -> Option<&PollAnswer> {
        match self {
            Self::PollAnswer { poll_answer } => Some(poll_answer),
            _ => None,
        }
    }

    /// Gets the "my chat member update" associated with this update, if any.
    pub fn my_chat_member(&self) -> Option<&ChatMemberUpdated> {
        match self {
            Self::MyChatMemberUpdated { my_chat_member } => Some(my_chat_member),
            _ => None,
        }
    }

    /// Gets the "chat member update" associated with this update, if any.
    pub fn chat_member(&self) -> Option<&ChatMemberUpdated> {
        match self {
            Self::ChatMemberUpdated { chat_member } => Some(chat_member),
            _ => None,
        }
    }

    /// `true` if it is a message update.
    pub fn is_message(&self) -> bool {
        matches!(self, Self::Message { .. })
    }

    /// `true` if it is a edited message update.
    pub fn is_edited_message(&self) -> bool {
        matches!(self, Self::EditedMessage { .. })
    }

    /// `true` if it is a channel post update.
    pub fn is_channel_post(&self) -> bool {
        matches!(self, Self::ChannelPost { .. })
    }

    /// `true` if it is a edited channel post update.
    pub fn is_edited_channel_post(&self) -> bool {
        matches!(self, Self::EditedChannelPost { .. })
    }

    /// `true` if it is a inline query update.
    pub fn is_inline_query(&self) -> bool {
        matches!(self, Self::InlineQuery { .. })
    }

    /// `true` if it is a chosen inline result update.
    pub fn is_chosen_inline_result(&self) -> bool {
        matches!(self, Self::ChosenInlineResult { .. })
    }

    /// `true` if it is a callback query update.
    pub fn is_callback_query(&self) -> bool {
        matches!(self, Self::CallbackQuery { .. })
    }

    /// `true` if it is a shipping query update.
    pub fn is_shipping_query(&self) -> bool {
        matches!(self, Self::ShippingQuery { .. })
    }

    /// `true` if it is a pre checkout query update.
    pub fn is_pre_checkout_query(&self) -> bool {
        matches!(self, Self::PreCheckoutQuery { .. })
    }

    /// `true` if it is a poll update.
    pub fn is_poll(&self) -> bool {
        matches!(self, Self::Poll { .. })
    }

    /// `true` if it is a poll answer update.
    pub fn is_poll_answer(&self) -> bool {
        matches!(self, Self::PollAnswer { .. })
    }

    /// `true` if it is a my chat member update update.
    pub fn is_my_chat_member_updated(&self) -> bool {
        matches!(self, Self::MyChatMemberUpdated { .. })
    }

    /// `true` if it is a chat member update.
    pub fn is_chat_member_updated(&self) -> bool {
        matches!(self, Self::ChatMemberUpdated { .. })
    }
}

/// Receives incoming updates using long polling ([wiki](https://en.wikipedia.org/wiki/Push_technology#Long_polling)).
///
/// [*Documentation on Telegram API Docs*](https://core.telegram.org/bots/api#getupdates)
#[derive(Clone, Serialize)]
pub struct GetUpdates {
    /// Identifier of the first update to be returned.
    /// Must be greater by one than the highest among the identifiers of previously received updates.
    /// By default, updates starting with the earliest unconfirmed update are returned.
    /// An update is considered confirmed as soon as getUpdates is called with an offset higher than its update_id.
    /// The negative offset can be specified to retrieve updates starting from -offset update from the end of the updates queue.
    /// All previous updates will forgotten.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Limits the number of updates to be retrieved.
    /// Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Timeout in seconds for long polling.
    /// Defaults to 0, i.e. usual short polling.
    /// Should be positive, short polling should be used for testing purposes only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    /// A JSON-serialized list of the update types you want your bot to receive.
    /// For example, specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types.
    /// See [Update](https://core.telegram.org/bots/api#update) for a complete list of available update types.
    /// Specify an empty list to receive all update types except chat_member (default).
    /// If not specified, the previous setting will be used.
    ///
    /// Please note that this parameter doesn't affect updates created before the call to the getUpdates,
    /// so unwanted updates may be received for a short period of time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

impl GetUpdates {
    /// Create a new [`GetUpdates`] request.
    pub fn new() -> Self {
        Self {
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }
    /// Sets offset.
    pub fn with_offset(self, offset: i32) -> Self {
        Self {
            offset: Some(offset),
            ..self
        }
    }
    /// Sets limit.
    pub fn with_limit(self, limit: u32) -> Self {
        Self {
            limit: Some(limit),
            ..self
        }
    }
    /// Sets timeout.
    pub fn with_timeout(self, timeout: u32) -> Self {
        Self {
            timeout: Some(timeout),
            ..self
        }
    }
    /// Sets allowed updates.
    pub fn with_allowed_updates(self, updates: Vec<String>) -> Self {
        Self {
            allowed_updates: Some(updates),
            ..self
        }
    }
    /// Adds one allowed update.
    pub fn with_allowed_update(mut self, update: impl Into<String>) -> Self {
        let updates = self.allowed_updates.get_or_insert_with(Default::default);
        updates.push(update.into());
        Self {
            allowed_updates: self.allowed_updates,
            ..self
        }
    }
}

impl TelegramMethod for GetUpdates {
    type Response = Vec<Update>;

    fn name() -> &'static str {
        "getUpdates"
    }
}

impl JsonMethod for GetUpdates {}
