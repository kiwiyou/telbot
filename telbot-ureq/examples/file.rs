use std::env;

use telbot_types::file::InputFile;
use telbot_ureq::polling::Polling;
use telbot_ureq::Api;

fn main() {
    let api = Api::new(env::var("BOT_TOKEN").unwrap());
    let kiwi = include_bytes!("kiwi.jpg");

    for update in Polling::new(&api) {
        let update = update.unwrap();
        if let Some(message) = update.kind.message() {
            if matches!(message.kind.text(), Some(text) if text.starts_with("/start")) {
                api.send_file(&message.chat.send_photo(InputFile {
                    name: "kiwi.jpg".to_string(),
                    data: kiwi.to_vec(),
                    mime: "image/jpg".to_string(),
                }))
                .unwrap();
            }
        }
    }
}
