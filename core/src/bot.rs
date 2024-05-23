use crate::{
    models::{
        allowed_update::AllowedUpdate,
        update::{Update, UpdateContent},
    },
    params::{message_params::SendMessageParams, updates_params::GetUpdatesParamsBuilder},
    BotCommands, CommandHandler, EventHandler, JsonData, RateLimiter, TelegrapherError,
    TelegrapherResult, UpdateHandler,
};
use axum::{
    extract::State,
    routing::{get, post},
};
use axum::{response::IntoResponse, Extension};
use axum::{Json, Router};
use reqwest::StatusCode;
use serde_json::json;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tower_http::cors::{Any, CorsLayer};

#[must_use]
#[derive(Clone)]
pub struct Bot {
    pub token: String,
    pub handler: EventHandler,
    pub rate_limiter: Arc<RateLimiter>,
    pub message_sender: Arc<Option<mpsc::Sender<SendMessageParams>>>,
}

impl Bot {
    pub fn new(token: &str) -> Self {
        let token = token.to_string();
        Self {
            token,
            handler: EventHandler::default(),
            rate_limiter: Arc::new(RateLimiter::new()),
            message_sender: Arc::new(None),
        }
    }

    fn set_message_sender(&mut self, sender: mpsc::Sender<SendMessageParams>) {
        self.message_sender = Arc::new(Some(sender));
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn register_update_handler(&mut self, handler: UpdateHandler) {
        self.handler.register_update_handler(handler);
    }

    pub fn register_commands_handler<T: BotCommands>(&mut self, handler: CommandHandler) {
        self.handler.register_command_handler::<T>(handler);
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
                eprintln!("Failed to build GetUpdatesParams");
                continue;
            }
            let params = params.unwrap();
            let response = self.get_updates(&params).await;
            if let Err(e) = response {
                eprintln!("{}", e);
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
                    eprintln!("No updates found: {:?}", updates_res.description);
                }
            }
        }
    }

    /// Start getting updates with Webhook.
    pub async fn start_webhook(&mut self, addr: &str) -> Result<(), TelegrapherError> {
        // Start message sender
        let (sender, mut receiver) = mpsc::channel(2048);
        let (sleep_sender, mut sleep_receiver) = mpsc::channel::<u64>(64);
        self.set_message_sender(sender);
        let bot = self.clone();
        tokio::spawn(async move {
            while let Some(params) = receiver.recv().await {
                let bot = bot.clone();
                let sleep_sender = sleep_sender.clone();
                tokio::spawn(async move {
                    match bot.send_message(&params).await {
                        Ok(response) => {
                            if !response.ok {
                                let parameter = response.parameters;
                                if parameter.is_none() {
                                    return;
                                }
                                let parameter = parameter.unwrap();
                                if parameter.retry_after.is_none() {
                                    return;
                                }

                                let retry_after = parameter.retry_after.unwrap();
                                _ = sleep_sender.send(retry_after).await;
                            }
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                        }
                    }
                });

                if let Ok(retry_after) = sleep_receiver.try_recv() {
                    tokio::time::sleep(tokio::time::Duration::from_secs(retry_after + 1)).await;
                }
            }
        });

        let app = self.new_router();
        let listener = tokio::net::TcpListener::bind(addr).await?;
        let addr = listener.local_addr().expect("failed to get local addr");
        println!("Webhook is running on {}", addr);
        match axum::serve(listener, app).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}

impl Bot {
    fn new_router(&self) -> Router {
        let router = Router::new()
            .route("/ping", get(Self::ping))
            .route("/webhook", post(Self::webhook_update))
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any),
            )
            .layer(Extension(self.clone()));
        return router;
    }

    async fn ping() -> &'static str {
        "pong"
    }

    async fn webhook_update(
        Extension(bot): Extension<Bot>,
        Json(update): Json<Update>,
    ) -> impl IntoResponse {
        if let Ok(json_data) = bot.process_update(&update.content).await {
            if let Some(data) = json_data {
                return Json(data);
            }
        }
        return Json(json!({}));
    }

    async fn process_update(&self, content: &UpdateContent) -> TelegrapherResult<Option<JsonData>> {
        match content {
            UpdateContent::Message(message) => {
                // if the content of message is a command
                if let Some(text) = message.text.as_ref() {
                    if text.starts_with('/') {
                        let command = text.split_whitespace().next().unwrap();
                        if self.handler.commands.contains(&command.to_string()) {
                            if let Some(handler) = self.handler.command_handler {
                                return handler(self.clone(), message.clone(), command.to_string())
                                    .await;
                            }
                        }
                    }
                }
                if let Some(handler) = self.handler.update_handler {
                    return handler(self.clone(), content.clone()).await;
                }
            }
            _ => {
                if let Some(handler) = self.handler.update_handler {
                    return handler(self.clone(), content.clone()).await;
                }
            }
        }
        return Ok(None);
    }
}
