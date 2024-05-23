use bot::Bot;
use models::{message::Message, update::UpdateContent};
use std::future::Future;
use std::pin::Pin;

pub mod bot;
pub mod methods;
pub mod models;
pub mod params;
pub mod requests;
pub mod responses;

pub const TELEGRAM_API_URL: &str = "https://api.telegram.org";

pub type TelegrapherError = Box<dyn std::error::Error + Send + Sync>;
pub type TelegrapherResult<T> = Result<T, TelegrapherError>;
pub type JsonData = serde_json::Value;

pub trait BotCommands: Sized {
    fn command_name(&self) -> &'static str;

    fn to_name_vec() -> Vec<&'static str>;
}

type UpdateHandler =
    fn(
        Bot,
        UpdateContent,
    ) -> Pin<Box<dyn Future<Output = TelegrapherResult<Option<JsonData>>> + Send>>;

pub type CommandHandler =
    fn(
        Bot,
        Message,
        String,
    ) -> Pin<Box<dyn Future<Output = TelegrapherResult<Option<JsonData>>> + Send>>;

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

    pub fn register_command_handler<T: BotCommands>(&mut self, handler: CommandHandler) {
        self.command_handler = Some(handler);
        for cmd in T::to_name_vec() {
            self.commands.push(cmd.to_string());
        }
    }
}
