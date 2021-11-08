use std::env;

use telbot_types::file::InputFile;
use telbot_types::message::SendPhoto;
use telbot_types::update::GetUpdates;
use telbot_ureq::Api;

fn main() {
    let api = Api::new(env::var("BOT_TOKEN").unwrap());
    let kiwi = include_bytes!("kiwi.jpg");

    let mut offset = 0;
    loop {
        let request = GetUpdates::new().with_timeout(1).with_offset(offset);
        let updates = api.send_json(&request).unwrap();
        for update in updates {
            offset = offset.max(update.update_id as i32 + 1);
            if let Some(message) = update.kind.message() {
                if matches!(message.kind.text(), Some(text) if text.starts_with("/start")) {
                    api.send_file(&SendPhoto::new(
                        message.chat.id,
                        InputFile {
                            name: "kiwi.jpg".to_string(),
                            data: kiwi.to_vec(),
                            mime: "image/jpg".to_string(),
                        },
                    ))
                    .unwrap();
                }
            }
        }
    }
}
