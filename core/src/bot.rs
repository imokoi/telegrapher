use std::time::Duration;
use std::{borrow::BorrowMut, cell::RefCell, ops::DerefMut, sync::Arc};

use axum::{
    handler,
    routing::{get, post},
};
use axum::{response::IntoResponse, Extension};
use axum::{Json, Router};
use serde_json::json;
use tokio::sync::{mpsc, Mutex};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    models::{
        allowed_update::AllowedUpdate,
        update::{Update, UpdateContent},
    },
    params::{message_params::SendMessageParams, updates_params::GetUpdatesParamsBuilder},
    BotCommands, CommandHandler, EventHandler, JsonData, MessageSendLockTime, RateLimitSemaphore,
    TelegrapherError, TelegrapherResult, UpdateHandler,
};

#[must_use]
#[derive(Clone)]
pub struct Bot {
    pub token: Arc<String>,
    pub handler: Arc<Mutex<EventHandler>>,
    pub rate_limiter: Arc<RateLimitSemaphore>,
    pub message_send_lock_time: Arc<MessageSendLockTime>,
    pub message_sender: Arc<Mutex<Option<mpsc::Sender<SendMessageParams>>>>,
}

impl Bot {
    pub fn new(token: &str, instance_count: i64) -> Self {
        let default_sender_sleep_times = MessageSendLockTime::default();
        let new_sender_sleep_times = MessageSendLockTime {
            global: default_sender_sleep_times.global * instance_count as f64,
            user_chat: default_sender_sleep_times.user_chat * instance_count as f64,
            group_chat: default_sender_sleep_times.group_chat * instance_count as f64,
        };

        Self {
            token: Arc::new(token.to_string()),
            handler: Arc::new(Mutex::new(EventHandler::default())),
            rate_limiter: Arc::new(RateLimitSemaphore::default()),
            message_send_lock_time: Arc::new(new_sender_sleep_times),
            message_sender: Arc::new(Mutex::new(None)),
        }
    }

    /// set the message sender, this is a channel sender.
    /// send message to the channel, then bot get message from channel and send it.
    async fn set_message_sender(&self, sender: mpsc::Sender<SendMessageParams>) {
        let mut message_sender = self.message_sender.lock().await;
        *message_sender = Some(sender);
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    /// register update handler function.
    pub async fn register_update_handler(&self, handler: UpdateHandler) {
        self.handler.lock().await.register_update_handler(handler);
    }

    /// register command handler function.
    pub async fn register_commands_handler<T: BotCommands>(&self, handler: CommandHandler) {
        self.handler
            .lock()
            .await
            .register_command_handler::<T>(handler);
    }

    /// Start getting updates from telegram api server.
    pub async fn start(&self) -> Result<(), TelegrapherError> {
        let mut offset = None;
        loop {
            let params = GetUpdatesParamsBuilder::default()
                .offset(offset)
                .timeout(10)
                .allowed_updates(vec![AllowedUpdate::Message, AllowedUpdate::CallbackQuery])
                .build();
            if let Err(_) = params {
                log::error!("failed to build params");
                continue;
            }
            let params = params.unwrap();
            let response = self.get_updates(&params).await;
            if let Err(e) = response {
                log::error!("failed to get updates: {:?}", e);
                continue;
            }

            let updates_res = response.unwrap();
            let result = updates_res.result;
            match result {
                Some(mut updates) => {
                    updates.sort_by(|a, b| a.update_id.cmp(&b.update_id));
                    for update in updates {
                        offset = Some(update.update_id + 1);
                        let bot = Arc::new(self.clone());
                        tokio::spawn(async move {
                            _ = bot.process_update(&update.content).await;
                        });
                    }
                }
                None => {
                    log::error!("no updates found: {:?}", updates_res.description);
                }
            }
        }
    }

    /// Start getting updates with Webhook.
    pub async fn start_webhook(&self, addr: &str) -> Result<(), TelegrapherError> {
        let bot = self.clone();
        tokio::spawn(async move {
            let _ = bot.start_message_channel_monitor().await;
        });

        let app = self.new_router();
        let listener = tokio::net::TcpListener::bind(addr).await?;
        let addr = listener.local_addr().expect("failed to get local addr");
        log::info!("Webhook is running on {}", addr);
        match axum::serve(listener, app).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    /// Start a message channel monitor to send messages to telegram api server.
    /// This method can catch retry_after parameter then wait for that time then resend the message.
    pub async fn start_message_channel_monitor(&self) {
        // create a channel with buffer.
        let (sender, mut receiver) = mpsc::channel(2048);
        self.set_message_sender(sender.clone()).await;
        let sleep_time = Arc::new(Mutex::new(0u64));
        // get message from channel.
        while let Some(params) = receiver.recv().await {
            let sleep_time_clone = sleep_time.clone();
            {
                let sleep_time = sleep_time_clone.lock().await;
                log::debug!("Sleeping for {} seconds", *sleep_time);
                if *sleep_time > 0 {
                    tokio::time::sleep(tokio::time::Duration::from_secs(*sleep_time)).await;
                }
            }
            {
                let mut sleep_time = sleep_time_clone.lock().await;
                *sleep_time = 0;
            }

            let bot = self.clone();
            let channel_sender = sender.clone();
            tokio::spawn(async move {
                match bot.send_message_throttled(&params).await {
                    Ok(response) => {
                        if response.ok {
                            return;
                        }
                        if let Some(retry_after) = response.parameters.and_then(|p| p.retry_after) {
                            {
                                let mut sleep_time = sleep_time_clone.lock().await;
                                *sleep_time += retry_after;
                            }
                            _ = channel_sender.send(params.clone()).await;
                        }
                    }
                    Err(e) => {
                        log::error!("failed to send message: {:?}", e);
                        _ = channel_sender.send(params.clone()).await;
                    }
                }
            });
        }
    }

    pub async fn get_message_send_permissions(
        &self,
        chat_id: i64,
    ) -> Result<bool, TelegrapherError> {
        if chat_id > 0 {
            let rate_limiter = self.rate_limiter.clone();
            let user_chat_sem = rate_limiter.acquire_user_chat(chat_id).await;
            let user_chat_permit = user_chat_sem.acquire_owned().await;
            if user_chat_permit.is_err() {
                return Err("User chat semaphore error".into());
            }

            let global_chat_sem = rate_limiter.acquire_global().await;
            let global_chat_permit = global_chat_sem.acquire_owned().await;
            if global_chat_permit.is_err() {
                return Err("Global chat semaphore error".into());
            }

            let self_clone = self.clone();
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs_f64(
                    self_clone.message_send_lock_time.global,
                ))
                .await;
                drop(global_chat_permit);

                tokio::time::sleep(Duration::from_secs_f64(
                    self_clone.message_send_lock_time.user_chat,
                ))
                .await;
                drop(user_chat_permit);
            });
        } else {
            let rate_limiter = self.rate_limiter.clone();
            let group_chat_sem = rate_limiter.acquire_user_chat(chat_id).await;
            let group_chat_permit = group_chat_sem.acquire_owned().await;
            if group_chat_permit.is_err() {
                return Err("Group chat semaphore error".into());
            }

            let global_chat_sem = rate_limiter.acquire_global().await.clone();
            let global_chat_permit = global_chat_sem.acquire_owned().await;
            if global_chat_permit.is_err() {
                return Err("Global chat semaphore error".into());
            }

            let self_clone = self.clone();
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs_f64(
                    self_clone.message_send_lock_time.global,
                ))
                .await;
                drop(global_chat_permit);

                tokio::time::sleep(Duration::from_secs_f64(
                    self_clone.message_send_lock_time.group_chat,
                ))
                .await;
                drop(group_chat_permit);
            });
        }

        Ok(true)
    }
}

impl Bot {
    fn new_router(&self) -> Router {
        Router::new()
            .route("/ping", get(Self::ping))
            .route("/webhook", post(Self::webhook_update))
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any),
            )
            .layer(Extension(self.clone()))
    }

    async fn ping() -> &'static str {
        "pong"
    }

    async fn webhook_update(
        Extension(bot): Extension<Bot>,
        Json(update): Json<Update>,
    ) -> impl IntoResponse {
        if let Ok(Some(json_data)) = bot.process_update(&update.content).await {
            return Json(json_data);
        }
        Json(json!({}))
    }

    async fn process_update(&self, content: &UpdateContent) -> TelegrapherResult<Option<JsonData>> {
        let handler;
        {
            handler = self.handler.lock().await;
        }
        match content {
            UpdateContent::Message(message) => {
                // if the content of message is a command
                if let Some(text) = message.text.as_ref() {
                    if text.starts_with('/') {
                        let command = text.split_whitespace().next().unwrap();
                        if handler.commands.contains(&command.to_string()) {
                            if let Some(handler) = handler.command_handler {
                                return handler(self.clone(), message.clone(), command.to_string())
                                    .await;
                            }
                        }
                    }
                }
                if let Some(handler) = handler.update_handler {
                    return handler(self.clone(), content.clone()).await;
                }
            }
            _ => {
                if let Some(handler) = handler.update_handler {
                    return handler(self.clone(), content.clone()).await;
                }
            }
        }
        Ok(None)
    }
}
