use core::{bot::Bot, models::message::Message, BotCommands, TelegramResult};
use macros::BotCommands;

#[tokio::main]
async fn main() {
    let bot = Bot::new("6616659571:AAEr0TdwPXBnvHQl_VJj5Z6wh-p3uUDNbOw");
    // bot.register_commands(BotCommand, handler);

    let commands = Commands::Help;
    let command = commands.command_name();
    println!("{}", command);

    bot.start().await.unwrap();
}

#[derive(BotCommands)]
pub enum Commands {
    Help,
    Menu,
    About,
}

async fn command_handler(bot: Bot, msg: Message, cmd: Commands) -> TelegramResult<()> {
    match cmd {
        Commands::Help => {}
        Commands::Menu => {}
        Commands::About => {}
    };
    Ok(())
}
