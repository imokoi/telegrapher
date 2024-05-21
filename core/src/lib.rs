use std::cell::RefCell;

use bot::Bot;
use models::update::UpdateContent;
use once_cell::sync::OnceCell;

pub mod bot;
pub mod methods;
pub mod models;
pub mod params;
pub mod requests;
pub mod responses;

pub type TelegramError = Box<dyn std::error::Error + Send + Sync>;

pub type TelegramResult<T> = Result<T, TelegramError>;
pub trait BotCommands {
    fn command_name(&self) -> &'static str;

    // fn to_name_vec() -> Vec<&'static str>;
}

type UpdateHandler = fn(bot: Bot, content: UpdateContent) -> TelegramResult<()>;
type CommandHandler = fn(bot: Bot, content: UpdateContent) -> TelegramResult<()>;

#[derive(Debug, Default, Clone)]
pub struct EventHandler {
    pub commands: Vec<String>,
    pub update_handler: Option<UpdateHandler>,
    pub command_handler: Option<CommandHandler>,
}

impl EventHandler {
    pub fn register_update_handler(&mut self, handler: UpdateHandler) {
        self.update_handler = Some(handler);
    }

    pub fn register_command_handler(
        &mut self,
        commands: impl BotCommands,
        handler: CommandHandler,
    ) {
        self.command_handler = Some(handler);
        self.commands.push(commands.command_name().to_string());
    }
}
