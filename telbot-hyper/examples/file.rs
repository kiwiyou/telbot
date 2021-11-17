use std::env;

use telbot_hyper::{
    types::{file::InputFile, update::GetUpdates},
    Api,
};

#[tokio::main]
async fn main() {
    let api = Api::new(env::var("BOT_TOKEN").unwrap());

    let mut offset = 0;
    loop {
        let get_updates = GetUpdates::new().with_offset(offset as i32).with_timeout(1);

        let updates = api.send_json(&get_updates).await.unwrap();
        for update in updates {
            if let Some(message) = update.kind.message() {
                if matches!(message.kind.text(), Some(text) if text.starts_with("/start")) {
                    let photo = InputFile {
                        name: "clover.jpg".to_string(),
                        data: include_bytes!("clover.jpg").to_vec(),
                        mime: "image/jpg".to_string(),
                    };
                    let request = message.chat.send_photo(photo);
                    let api = api.clone();
                    tokio::spawn(async move {
                        api.send_file(&request).await.unwrap();
                    });
                }
            }
            offset = offset.max(update.update_id + 1);
        }
    }
}
