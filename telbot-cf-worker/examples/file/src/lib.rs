use telbot_cf_worker::types::file::InputFile;
use telbot_cf_worker::types::message::SendPhoto;
use telbot_cf_worker::types::update::{Update, UpdateKind};
use worker::*;

mod utils;

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    utils::set_panic_hook();

    let api = telbot_cf_worker::Api::new(env.secret("BOT_TOKEN").unwrap().to_string());
    let router = Router::with_data(api);

    router
        .post_async("/", |mut req, ctx| async move {
            let update = req.json::<Update>().await.unwrap();
            if let UpdateKind::Message { message } = update.kind {
                if matches!(message.text(), Some(text) if text.starts_with("/start")) {
                    let clover = include_bytes!("../clover.jpg");
                    let api = ctx.data();
                    api.send_file(&SendPhoto::new(
                        message.chat.id,
                        InputFile {
                            name: "clover.jpg".to_string(),
                            data: clover.to_vec(),
                            mime: "image/jpg".to_string(),
                        },
                    ))
                    .await
                    .expect("failed to send message");
                }
            }
            Response::empty()
        })
        .run(req, env)
        .await
}
