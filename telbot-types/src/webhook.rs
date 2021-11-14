use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::file::InputFile;
use crate::{FileMethod, JsonMethod, TelegramMethod};

/// Contains information about the current status of a webhook.
#[derive(Debug, Deserialize)]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up
    pub url: String,
    /// True, if a custom certificate was provided for webhook certificate checks
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery
    pub pending_update_count: u32,
    /// Currently used webhook IP address
    pub ip_address: Option<String>,
    /// Unix time for the most recent error that happened when trying to deliver an update via webhook
    pub last_error_date: u64,
    /// Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    pub last_error_message: Option<String>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    pub max_connections: Option<u32>,
    /// A list of update types the bot is subscribed to.
    /// Defaults to all update types except chat_member
    pub allowed_updates: Option<Vec<String>>,
}

/// Use this method to specify a url and receive incoming updates via an outgoing webhook.
/// Whenever there is an update for the bot, we will send an HTTPS POST request to the specified url, containing a JSON-serialized [Update](https://core.telegram.org/bots/api#update).
/// In case of an unsuccessful request, we will give up after a reasonable amount of attempts.
/// Returns True on success.
///
/// If you'd like to make sure that the Webhook request comes from Telegram,
/// we recommend using a secret path in the URL, e.g. `https://www.example.com/<token>`.
/// Since nobody else knows your bot's token, you can be pretty sure it's us.
#[derive(Clone, Serialize)]
pub struct SetWebhook {
    /// HTTPS url to send updates to. Use an empty string to remove webhook integration
    pub url: String,
    /// Upload your public key certificate so that the root certificate in use can be checked.
    /// See Telegram's [self-signed guide](https://core.telegram.org/bots/self-signed) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<InputFile>,
    /// The fixed IP address which will be used to send webhook requests instead of the IP address resolved through DNS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to 40.
    /// Use lower values to limit the load on your bot's server, and higher values to increase your bot's throughput.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u32>,
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
    /// Pass True to drop all pending updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}

impl SetWebhook {
    /// Create a new setWebhook request that sets the webhook url.
    pub fn new(url: String) -> Self {
        Self {
            url,
            certificate: None,
            ip_address: None,
            max_connections: None,
            allowed_updates: None,
            drop_pending_updates: None,
        }
    }
    /// Create a new setWebhook request that removes previous webhook.
    pub fn remove_previous() -> Self {
        Self {
            url: "".to_string(),
            certificate: None,
            ip_address: None,
            max_connections: None,
            allowed_updates: None,
            drop_pending_updates: None,
        }
    }
    /// Set custom certificate for the webhook
    pub fn with_certificate(self, cert: InputFile) -> Self {
        Self {
            certificate: Some(cert),
            ..self
        }
    }
    /// Set ip address to be used to send webhook request
    pub fn with_ip_address(self, ip_address: impl Into<String>) -> Self {
        Self {
            ip_address: Some(ip_address.into()),
            ..self
        }
    }
    /// Set maximum simultaneous webhook request count
    pub fn with_max_connections(self, max_connections: u32) -> Self {
        Self {
            max_connections: Some(max_connections),
            ..self
        }
    }
    /// Drop all pending updates
    pub fn drop_pending_updates(self) -> Self {
        Self {
            drop_pending_updates: Some(true),
            ..self
        }
    }
}

impl TelegramMethod for SetWebhook {
    type Response = bool;

    fn name() -> &'static str {
        "setWebhook"
    }
}

impl FileMethod for SetWebhook {
    fn files(&self) -> Option<std::collections::HashMap<&str, &InputFile>> {
        self.certificate.as_ref().map(|file| {
            let mut map = HashMap::new();
            map.insert("certificate", file);
            map
        })
    }
}

/// Use this method to remove webhook integration if you decide to switch back to [getUpdates](https://core.telegram.org/bots/api#getupdates).
/// Returns True on success.
#[derive(Clone, Serialize)]
pub struct DeleteWebhook {
    /// Pass True to drop all pending updates
    pub drop_pending_updates: Option<bool>,
}

impl DeleteWebhook {
    pub fn new() -> Self {
        Self {
            drop_pending_updates: None,
        }
    }
    /// Drop all pending updates
    pub fn drop_pending_updates(self) -> Self {
        Self {
            drop_pending_updates: Some(true),
            ..self
        }
    }
}

impl TelegramMethod for DeleteWebhook {
    type Response = bool;

    fn name() -> &'static str {
        "deleteWebhook"
    }
}

impl JsonMethod for DeleteWebhook {}

/// Use this method to get current webhook status. Requires no parameters.
/// On success, returns a WebhookInfo object.
/// If the bot is using getUpdates, will return an object with the url field empty.
#[derive(Clone, Serialize)]
pub struct GetWebhookInfo;

impl TelegramMethod for GetWebhookInfo {
    type Response = WebhookInfo;

    fn name() -> &'static str {
        "getWebhookInfo"
    }
}

impl JsonMethod for GetWebhookInfo {}
