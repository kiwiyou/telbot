use crate::chat::ChatId;
use crate::user::User;
use crate::{JsonMethod, TelegramMethod};
use serde::{Deserialize, Serialize};

/// This object represents a bot command.
#[derive(Serialize, Deserialize, Clone)]
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
#[derive(Serialize)]
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

/// Use this method to change the list of the bot's commands. See https://core.telegram.org/bots#commands for more details about bot commands.
/// Returns _True_ on success.
#[derive(Serialize)]
pub struct SetMyCommands {
    /// A JSON-serialized list of bot commands to be set as the list of the bot's commands.
    /// At most 100 commands can be specified.
    pub commands: Vec<BotCommand>,
    /// A JSON-serialized object, describing scope of users for which the commands are relevant.
    /// Defaults to [BotCommandScopeDefault](https://core.telegram.org/bots/api#botcommandscopedefault).
    pub scope: Option<BotCommandScope>,
    /// A two-letter ISO 639-1 language code.
    /// If empty, commands will be applied to all users from the given scope,
    /// for whose language there are no dedicated commands
    pub language_code: Option<String>,
}

impl SetMyCommands {
    /// Create a new setMyCommands request
    pub fn new(commands: impl Into<Vec<BotCommand>>) -> Self {
        Self {
            commands: commands.into(),
            scope: None,
            language_code: None,
        }
    }

    /// Set command scope
    pub fn with_scope(self, scope: BotCommandScope) -> Self {
        Self {
            scope: Some(scope),
            ..self
        }
    }

    /// Set language code
    pub fn with_language_code(self, language_code: impl Into<String>) -> Self {
        Self {
            language_code: Some(language_code.into()),
            ..self
        }
    }
}

impl TelegramMethod for SetMyCommands {
    type Response = bool;

    fn name() -> &'static str {
        "setMyCommands"
    }
}

impl JsonMethod for SetMyCommands {}

/// Use this method to delete the list of the bot's commands for the given scope and user language.
/// After deletion, higher level commands will be shown to affected users.
/// Returns _True_ on success.
#[derive(Serialize)]
pub struct DeleteMyCommands {
    /// A JSON-serialized object, describing scope of users for which the commands are relevant.
    /// Defaults to [BotCommandScopeDefault](https://core.telegram.org/bots/api#botcommandscopedefault).
    pub scope: Option<BotCommandScope>,
    /// A two-letter ISO 639-1 language code.
    /// If empty, commands will be applied to all users from the given scope,
    /// for whose language there are no dedicated commands
    pub language_code: Option<String>,
}

impl DeleteMyCommands {
    /// Create a new deleteMyCommands request
    pub fn new() -> Self {
        Self {
            scope: None,
            language_code: None,
        }
    }
    /// Set command scope
    pub fn with_scope(self, scope: BotCommandScope) -> Self {
        Self {
            scope: Some(scope),
            ..self
        }
    }

    /// Set language code
    pub fn with_language_code(self, language_code: impl Into<String>) -> Self {
        Self {
            language_code: Some(language_code.into()),
            ..self
        }
    }
}

impl TelegramMethod for DeleteMyCommands {
    type Response = bool;

    fn name() -> &'static str {
        "deleteMyCommands"
    }
}

/// Use this method to get the current list of the bot's commands for the given scope and user language.
/// Returns Array of [BotCommand](https://core.telegram.org/bots/api#botcommand) on success.
/// If commands aren't set, an empty list is returned.
#[derive(Serialize)]
pub struct GetMyCommands {
    /// A JSON-serialized object, describing scope of users for which the commands are relevant.
    /// Defaults to [BotCommandScopeDefault](https://core.telegram.org/bots/api#botcommandscopedefault).
    pub scope: Option<BotCommandScope>,
    /// A two-letter ISO 639-1 language code.
    /// If empty, commands will be applied to all users from the given scope,
    /// for whose language there are no dedicated commands
    pub language_code: Option<String>,
}

impl GetMyCommands {
    /// Create a new getMyCommands request
    pub fn new() -> Self {
        Self {
            scope: None,
            language_code: None,
        }
    }
    /// Set command scope
    pub fn with_scope(self, scope: BotCommandScope) -> Self {
        Self {
            scope: Some(scope),
            ..self
        }
    }

    /// Set language code
    pub fn with_language_code(self, language_code: impl Into<String>) -> Self {
        Self {
            language_code: Some(language_code.into()),
            ..self
        }
    }
}

impl TelegramMethod for GetMyCommands {
    type Response = Vec<BotCommand>;

    fn name() -> &'static str {
        "getMyCommands"
    }
}
