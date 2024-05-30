use core::{
    bot::Bot,
    models::{message::Message, update::UpdateContent},
    BotCommands, JsonData, TelegrapherResult,
};
use macros::{event_handler, BotCommands};
use std::{future::Future, pin::Pin};

#[tokio::main]
async fn main() {
    let mut bot = Bot::new("6616659571:AAEr0TdwPXBnvHQl_VJj5Z6wh-p3uUDNbOw");
    // bot.register_commands(BotCommand, handler);

    let commands = Commands::HelpMessage;
    let command = commands.as_str();
    println!("{}", command);

    // let names = Commands::to_name_vec();
    // println!("{:?}", names);

    let str = "/helpmessage";
    let cmd = Commands::try_from(str);
    println!("{:?}", cmd);

    let commands = Commands::vec();
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
    };
    Ok(Option::None)
}

#[event_handler]
async fn update_handler(_bot: Bot, _update: UpdateContent) -> TelegrapherResult<Option<JsonData>> {
    println!("Update handler");
    Ok(Option::None)
}
