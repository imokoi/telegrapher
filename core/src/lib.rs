use std::collections::HashMap;
use std::fmt::Display;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::{Mutex, Semaphore};

use bot::Bot;
use models::{message::Message, update::UpdateContent};

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
    fn as_str(&self) -> &'static str;

    fn to_vec(enable_skip: bool) -> Vec<Self>;
}

/// Update handler type
type UpdateHandler =
    fn(
        Bot,
        UpdateContent,
    ) -> Pin<Box<dyn Future<Output = TelegrapherResult<Option<JsonData>>> + Send>>;

/// Command Handler type
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
        for cmd in T::to_vec(false) {
            self.commands.push(cmd.as_str().to_string());
        }
    }
}

#[derive(Debug)]
pub struct MessageSendLockTime {
    global: f64,
    user_chat: f64,
    group_chat: f64,
}

impl Default for MessageSendLockTime {
    fn default() -> Self {
        Self {
            global: 1.0 / 30.0,
            user_chat: 1.0,
            group_chat: 60.0 / 20.0,
        }
    }
}

/// Rate limiter for sending messages
#[derive(Debug)]
pub struct RateLimitSemaphore {
    global: Arc<Semaphore>,
    user_chat: Arc<Mutex<HashMap<i64, Arc<Semaphore>>>>,
    group_chat: Arc<Mutex<HashMap<String, Arc<Semaphore>>>>,
}

impl Default for RateLimitSemaphore {
    fn default() -> Self {
        Self {
            global: Arc::new(Semaphore::new(1)),
            user_chat: Arc::new(Mutex::new(HashMap::new())),
            group_chat: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

// 30 messages per second
// 1 message per second inside particular chat
// 20 messages per minute to the same group.
impl RateLimitSemaphore {
    pub async fn acquire_global(&self) -> Arc<Semaphore> {
        self.global.clone()
    }

    pub async fn acquire_user_chat(&self, chat_id: i64) -> Arc<Semaphore> {
        let mut user_chat_semaphores = self.user_chat.lock().await;
        let sem = {
            user_chat_semaphores
                .entry(chat_id)
                .or_insert_with(|| Arc::new(Semaphore::new(1)))
                .clone()
        };
        sem
    }

    pub async fn acquire_group_chat(&self, chat_id: String) -> Arc<Semaphore> {
        let mut group_chat_semaphores = self.group_chat.lock().await;
        let sem = {
            group_chat_semaphores
                .entry(chat_id)
                .or_insert_with(|| Arc::new(Semaphore::new(1)))
                .clone()
        };
        sem
    }
}

#[derive(Debug, Clone)]
pub enum FileType {
    Photo,
    Video,
    Document,
    Audio,
    Voice,
    Animation,
}

impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            FileType::Photo => "photo".to_string(),
            FileType::Video => "video".to_string(),
            FileType::Document => "document".to_string(),
            FileType::Audio => "audio".to_string(),
            FileType::Voice => "voice".to_string(),
            FileType::Animation => "animation".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[tokio::test]
    async fn test_rate_limiter() {
        let rate_limiter = Arc::new(RateLimitSemaphore::default());
        let mut tasks = Vec::new();
        for i in 0..10 {
            let rate_limiter = rate_limiter.clone();
            let task = tokio::spawn(async move {
                println!("Task {} ", i);
                // let sem = rate_limiter.acquire_user_chat(1).await;
                // let permit = sem.acquire().await.unwrap();
                // println!("{}, available permits {}", i, sem.available_permits());
                tokio::time::sleep(Duration::from_secs(10)).await;
                // drop(permit);
                // println!(
                //     "{} task done, available permits {}",
                //     i,
                //     sem.available_permits()
                // );
                println!("Task {} done", i);
            });
            tasks.push(task);
        }

        for task in tasks {
            task.await.unwrap();
        }
        // join_all(tasks).await;
    }
}
