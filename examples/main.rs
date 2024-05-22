use core::{
    bot::Bot,
    models::{
        message::Message,
        update::{Update, UpdateContent},
    },
    BotCommands, CommandHandler, TelegramResult,
};
use macros::{event_handler, BotCommands};
use serde::Serialize;
use std::{future::Future, pin::Pin};

#[tokio::main]
async fn main() {
    let mut bot = Bot::new("6616659571:AAEr0TdwPXBnvHQl_VJj5Z6wh-p3uUDNbOw");
    // bot.register_commands(BotCommand, handler);

    let commands = Commands::HelpMessage;
    let command = commands.command_name();
    println!("{}", command);

    let names = Commands::to_name_vec();
    println!("{:?}", names);

    let str = "/help_message";
    let cmd = Commands::try_from(str);
    println!("{:?}", cmd);

    bot.register_commands_handler::<Commands>(command_handler);
    bot.register_update_handler(update_handler);

    bot.start().await.unwrap();
}

#[derive(BotCommands, Debug)]
pub enum Commands {
    HelpMessage,
    Menu,
    About,
}

#[event_handler]
async fn command_handler(bot: Bot, msg: Message, cmd: String) -> TelegramResult<()> {
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
    Ok(())
}

#[event_handler]
async fn update_handler(bot: Bot, update: UpdateContent) -> TelegramResult<()> {
    println!("Update handler");
    Ok(())
}
