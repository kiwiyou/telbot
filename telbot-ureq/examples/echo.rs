use std::env;

use telbot_types::message::SendMessage;
use telbot_types::update::{GetUpdates, UpdateKind};
use telbot_ureq::Api;

fn main() {
    let api = Api::new(env::var("BOT_TOKEN").unwrap());

    let mut offset = 0;
    loop {
        let request = GetUpdates::new().with_timeout(1).with_offset(offset);
        let updates = api.send_json(&request).unwrap();
        for update in updates {
            offset = offset.max(update.update_id as i32 + 1);
            match update.kind {
                UpdateKind::Message { message } => {
                    if let Some(text) = message.text() {
                        api.send_json(
                            &SendMessage::new(message.chat.id, text).reply_to(message.message_id),
                        )
                        .unwrap();
                    }
                }
                _ => {}
            }
        }
    }
}
