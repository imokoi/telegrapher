use std::{future::Future, pin::Pin};

use core::{
    bot::Bot,
    BotCommands,
    JsonData, models::{message::Message, update::UpdateContent}, TelegrapherResult,
};
use macros::{BotCommands, event_handler};

#[tokio::main]
async fn main() {
    let mut bot = Bot::new("6616659571:AAEr0TdwPXBnvHQl_VJj5Z6wh-p3uUDNbOw", 1);
    // bot.register_commands(BotCommand, handler);

    let commands = Commands::HelpMessage;
    let command = commands.as_str();
    println!("{}", command);

    // let names = Commands::to_name_vec();
    // println!("{:?}", names);

    let str = "/helpmessage";
    let cmd = Commands::try_from(str);
    println!("{:?}", cmd);

    let commands_with_skip = Commands::to_vec(false);
    println!("{:?}", commands_with_skip);

    let commands = Commands::to_vec(true);
    println!("{:?}", commands);

    // bot.register_commands_handler::<Commands>(command_handler);
    // bot.register_update_handler(update_handler);

    // bot.start().await.unwrap();
}

#[derive(BotCommands, Debug)]
pub enum Commands {
    HelpMessage,
    Menu,
    About,
    #[BotCommands(skip)]
    Language,
}

// impl Commands {
//     pub fn iter() -> Vec<Self> {
//         vec![Commands::HelpMessage, Commands::Menu, Commands::About]
//     }
// }

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
        Commands::Language => {
            println!("Language command");
        }
    };
    Ok(Option::None)
}

#[event_handler]
async fn update_handler(_bot: Bot, _update: UpdateContent) -> TelegrapherResult<Option<JsonData>> {
    println!("Update handler");
    Ok(Option::None)
}
