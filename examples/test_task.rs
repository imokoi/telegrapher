use core::{bot::Bot, params::message_params::SendMessageParamsBuilder, RateLimiter};
use std::{sync::Arc, time::Instant};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let channel = tokio::sync::mpsc::channel::<i32>(100);

    let bot = Arc::new(Bot::new("token"));
    let rate_limiter = Arc::new(RateLimiter::new());
    // 模拟多个并发任务
    let mut tasks = Vec::new();
    for i in 0..30 {
        let my_bot = bot.clone();
        let task = tokio::spawn(async move {
            let chat_id = i % 3;
            request_api(&my_bot, chat_id).await;
        });
        tasks.push(task);
    }

    // 等待所有任务完成
    for task in tasks {
        task.await.unwrap();
    }
}

async fn request_api(bot: &Bot, chat_id: i32) {
    let params = SendMessageParamsBuilder::default()
        .text("Hello, world!")
        .chat_id(chat_id)
        .build()
        .unwrap();

    bot.send_message(true, &params).await;
    // let sem = rate_limiter.acquire_user_chat(chat_id).await;
    // let permit = sem.acquire().await.unwrap();
    // println!("{}, Requesting API", chat_id);
    // tokio::time::sleep(Duration::from_secs(3)).await;
    // drop(permit);
}
