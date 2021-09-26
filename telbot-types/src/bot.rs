use crate::chat::ChatId;
use crate::user::User;
use crate::{JsonMethod, TelegramMethod};
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
