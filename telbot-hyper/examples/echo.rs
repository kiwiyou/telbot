use std::env;

use telbot_hyper::{types::update::GetUpdates, Api};

#[tokio::main]
async fn main() {
    let api = Api::new(env::var("BOT_TOKEN").unwrap());

    let mut offset = 0;
    loop {
        let get_updates = GetUpdates::new().with_offset(offset as i32).with_timeout(1);

        let updates = api.send_json(&get_updates).await.unwrap();
        for update in updates {
            if let Some(message) = update.kind.message() {
                if let Some(text) = message.kind.text() {
                    let request = message.reply_text(text);
                    let api = api.clone();
                    tokio::spawn(async move {
                        api.send_json(&request).await.unwrap();
                    });
                }
            }
            offset = offset.max(update.update_id + 1);
        }
    }
}
