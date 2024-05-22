use std::{fmt::Debug, sync::Arc, time::Duration};

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::{
    models::{allowed_update::AllowedUpdate, update::UpdateContent},
    params::updates_params::GetUpdatesParamsBuilder,
    responses::MethodResponse,
    BotCommands, CommandHandler, EventHandler, TelegramError, UpdateHandler,
};

#[must_use]
#[derive(Debug, Clone)]
pub struct Bot {
    token: String,
    handler: EventHandler,
}

impl Bot {
    pub fn new(token: &str) -> Self {
        let token = token.to_string();
        Self {
            token,
            handler: EventHandler::default(),
        }
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
    pub async fn start(&self) -> Result<(), TelegramError> {
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

    async fn process_update(&self, content: &UpdateContent) {
        match content {
            UpdateContent::Message(message) => {
                // if the content of message is a command
                if let Some(text) = message.text.as_ref() {
                    if text.starts_with('/') {
                        let command = text.split_whitespace().next().unwrap();
                        if self.handler.commands.contains(&command.to_string()) {
                            if let Some(handler) = self.handler.command_handler {
                                _ = handler(self.clone(), message.clone(), command.to_string())
                                    .await;
                                return;
                            }
                        }
                    }
                }

                if let Some(handler) = self.handler.update_handler {
                    _ = handler(self.clone(), content.clone()).await;
                }
            }
            _ => {
                if let Some(handler) = self.handler.update_handler {
                    _ = handler(self.clone(), content.clone()).await;
                }
            }
        }
    }
}
