use core::{bot::Bot, params::message_params::SendMessageParamsBuilder};
use std::{sync::Arc};
use tokio::time::{Duration};

#[tokio::main]
async fn main() {
    let mut bot = Bot::new("6616659571:AAEr0TdwPXBnvHQl_VJj5Z6wh-p3uUDNbOw");
    bot.start_message_send_queue().await;

    // tokio::time::sleep(Duration::from_secs(5)).await;
    // let params = SendMessageParamsBuilder::default()
    //     .text("Hello")
    //     .chat_id(1393242628)
    //     .build()
    //     .unwrap();
    // bot.send_message_with_queue(&params).await;

    tokio::time::sleep(Duration::from_secs(5)).await;

    let arc_bot = Arc::new(bot);
    for i in 0..10000 {
        let my_bot = arc_bot.clone();
        // tokio::time::sleep(Duration::from_secs_f32(0.1)).await;
        let chat_id = 1393242628 + i;
        tokio::spawn(async move {
            request_api(&my_bot, chat_id).await;
        });
    }

    loop {
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    // let mut tasks = Vec::new();
    // for i in 0..1 {
    //     let my_bot = arc_bot.clone();
    //     let task = tokio::spawn(async move {
    //         let params = SendMessageParamsBuilder::default()
    //             .text("Hello")
    //             .chat_id(1393242628)
    //             .build()
    //             .unwrap();
    //         println!("my_bot: {:?}", my_bot.token());
    //         my_bot.send_message_with_queue(&params).await;
    //         // tokio::time::sleep(Duration::from_secs(5)).await;
    //     });
    //     tasks.push(task);
    // }

    // for task in tasks {
    //     task.await.unwrap();
    // }
}

async fn request_api(bot: &Bot, chat_id: i32) {
    let params = SendMessageParamsBuilder::default()
        .text("Hello")
        .chat_id(chat_id)
        .build()
        .unwrap();
    bot.send_message_with_queue(&params).await;
    // let sem = rate_limiter.acquire_user_chat(chat_id).await;
    // let permit = sem.acquire().await.unwrap();
    // println!("{}, Requesting API", chat_id);
    // tokio::time::sleep(Duration::from_secs(3)).await;
    // drop(permit);
}
