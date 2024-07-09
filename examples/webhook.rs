use std::{future::Future, pin::Pin};

use core::{
    bot::Bot,
    BotCommands,
    JsonData,
    models::{message::Message, update::UpdateContent},
    params::{message_params::SendMessageParamsBuilder, webhook_param::SetWebhookParamsBuilder}, responses::build_webhook_response, TelegrapherResult,
};
use macros::BotCommands;
use macros::event_handler;

#[tokio::main]
async fn main() {
    let bot = Bot::new("6616659571:AAEr0TdwPXBnvHQl_VJj5Z6wh-p3uUDNbOw", 1);
    let set_webhook_params = SetWebhookParamsBuilder::default()
        .url("https://namidev.com/webhook")
        .build()
        .unwrap();
    let result = bot.set_webhook(&set_webhook_params).await;
    println!("{:?}", result);

    let webhook_info = bot.get_webhook_info().await;
    println!("{:?}", webhook_info);

    bot.register_commands_handler::<Commands>(command_handler)
        .await;
    bot.register_update_handler(update_handler).await;
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
    _bot: Bot,
    _msg: Message,
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
        .chat_id(1393242628)
        .build()
        .unwrap();

    match build_webhook_response(method, params) {
        Ok(json) => Ok(Some(json)),
        Err(e) => Err(e),
    }
}
