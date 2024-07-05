use std::time::Duration;

use core::params::message_params::SendMessageParamsBuilder;
use telegrapher::bot::Bot;
use telegrapher::params::webhook_param::SetWebhookParamsBuilder;

#[tokio::main]
async fn main() {
    let bot = Bot::new("6508045653:AAHT_UnNWycb8HAzXuSeGzSWlaOVUpt_374");

    let bot_clone = bot.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(5)).await;
        let params = SendMessageParamsBuilder::default()
            .text("Hello")
            .chat_id(-1002198289004 as i64)
            .build()
            .unwrap();
        bot_clone.send_message_throttled_and_retry(&params).await;
    });

    let set_webhook_params = SetWebhookParamsBuilder::default()
        .url("https://test.com/webhook")
        .build()
        .unwrap();
    _ = bot.set_webhook(&set_webhook_params).await;
    bot.start_webhook("0.0.0.0:80").await.unwrap();
}
