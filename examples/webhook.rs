use core::{
    bot::Bot,
    models::{chat, message::Message, update::UpdateContent},
    params::{message_params::SendMessageParamsBuilder, webhook_param::SetWebhookParamsBuilder},
    responses::build_webhook_response,
    BotCommands, JsonData, TelegrapherResult,
};
use macros::event_handler;
use macros::BotCommands;
use std::{future::Future, pin::Pin};

#[tokio::main]
async fn main() {
    let mut bot = Bot::new("6616659571:AAEr0TdwPXBnvHQl_VJj5Z6wh-p3uUDNbOw");
    let set_webhook_params = SetWebhookParamsBuilder::default()
        .url("https://namidev.com/webhook")
        .build()
        .unwrap();
    let result = bot.set_webhook(&set_webhook_params).await;
    println!("{:?}", result);

    let webhook_info = bot.get_webhook_info().await;
    println!("{:?}", webhook_info);

    bot.register_commands_handler::<Commands>(command_handler);
    bot.register_update_handler(update_handler);
    bot.start_webhook("0.0.0.0:80").await.unwrap();
}

#[derive(BotCommands, Debug)]
pub enum Commands {
    HelpMessage,
    Menu,
    About,
}

#[event_handler]
async fn command_handler(
    bot: Bot,
    msg: Message,
    cmd: String,
) -> TelegrapherResult<Option<JsonData>> {
    let command = Commands::try_from(cmd.as_str())?;
    match command {
        Commands::HelpMessage => {
            println!("Help message command");
        }
        Commands::Menu => {}
        Commands::About => {
            println!("About command");
        }
    };
    Ok(Option::None)
}

#[event_handler]
async fn update_handler(bot: Bot, update: UpdateContent) -> TelegrapherResult<Option<JsonData>> {
    println!("{:?}", update);
    let method = "sendMessage";
    let params = SendMessageParamsBuilder::default()
        .text("Hello")
        .chat_id(-1002001406444 as i64)
        .message_thread_id(3)
        .build()
        .unwrap();
    bot.send_message_with_queue(&params).await;
    match build_webhook_response(method, params) {
        Ok(json) => Ok(Some(json)),
        Err(e) => Err(e),
    }
}
