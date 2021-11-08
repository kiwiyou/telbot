use std::env;

use telbot_ureq::polling::Polling;
use telbot_ureq::Api;

fn main() {
    let api = Api::new(env::var("BOT_TOKEN").unwrap());

    // Get updates by long polling
    for update in Polling::new(&api) {
        let update = update.unwrap();
        if let Some(message) = update.kind.message() {
            if let Some(text) = message.kind.text() {
                api.send_json(&message.reply_text(text)).unwrap();
            }
        }
    }
}
