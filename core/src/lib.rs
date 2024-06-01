use bot::Bot;
use futures::future::join_all;
use methods::user;
use models::{message::Message, update::UpdateContent};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, Semaphore};

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
        for cmd in T::to_vec(false) {
            self.commands.push(cmd.as_str().to_string());
        }
    }
}

pub struct RateLimiter {
    global_semaphore: Arc<Semaphore>,
    user_chat_semaphores: Arc<Mutex<HashMap<i64, Arc<Semaphore>>>>,
    group_chat_semaphores: Arc<Mutex<HashMap<String, Arc<Semaphore>>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            global_semaphore: Arc::new(Semaphore::new(1)),
            user_chat_semaphores: Arc::new(Mutex::new(HashMap::new())),
            group_chat_semaphores: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn acquire_global(&self) -> Arc<Semaphore> {
        self.global_semaphore.clone()
    }

    pub async fn acquire_user_chat(&self, chat_id: i64) -> Arc<Semaphore> {
        let mut user_chat_semaphores = self.user_chat_semaphores.lock().await;
        let sem = {
            user_chat_semaphores
                .entry(chat_id)
                .or_insert_with(|| Arc::new(Semaphore::new(1)))
                .clone()
        };
        sem
    }

    pub async fn acquire_group_chat(&self, chat_id: String) -> Arc<Semaphore> {
        let mut group_chat_semaphores = self.group_chat_semaphores.lock().await;
        let sem = {
            group_chat_semaphores
                .entry(chat_id)
                .or_insert_with(|| Arc::new(Semaphore::new(1)))
                .clone()
        };
        sem
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter() {
        let rate_limiter = Arc::new(RateLimiter::new());
        // 模拟多个并发任务
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

        // 等待所有任务完成
        for task in tasks {
            task.await.unwrap();
        }
        // join_all(tasks).await;
    }
}
