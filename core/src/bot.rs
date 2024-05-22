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
                // let button = InlineKeyboardButtonBuilder::default()
                //     .text("hello")
                //     .callback_data("hello callback".to_string())
                //     .build()
                //     .unwrap();

                // let inline_keyboards = vec![button];
                // let inline_keyboards_markup = InlineKeyboardMarkup {
                //     inline_keyboard: vec![inline_keyboards],
                // };
                // let inline_keyboards = ReplyMarkup::InlineKeyboardMarkup(inline_keyboards_markup);
                // let chat_id = message.chat.id;
                // let text = format!("You said: {}", message.text.as_ref().unwrap());
                // let param = params::send_message_params::SendMessageParamsBuilder::default()
                //     .chat_id(chat_id)
                //     .text(text.clone())
                //     .reply_markup(inline_keyboards)
                //     .build()?;
                // let response = self.send_message(&param).await?;

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
            // UpdateContent::CallbackQuery(callback_query) => {
            //     print!("{:?}", callback_query.message.as_ref().unwrap());
            //     if let MaybeInaccessibleMessage::Message(message) =
            //         callback_query.message.as_ref().unwrap()
            //     {
            //         let chat_id = message.chat.id;
            //         let text = format!("You clicked: {}", callback_query.data.as_ref().unwrap());
            //         let param = AnswerCallbackQueryParamsBuilder::default()
            //             .callback_query_id(callback_query.id.clone())
            //             .text(text)
            //             .build()?;
            //         let response = self.answer_callback_query(&param).await;
            //         println!("{:?}", response);
            //     }
            // }
            _ => {
                if let Some(handler) = self.handler.update_handler {
                    _ = handler(self.clone(), content.clone()).await;
                }
            }
        }
    }
}
