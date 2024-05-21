use std::{fmt::Debug, sync::Arc, time::Duration};

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::{
    models::{
        allowed_update::AllowedUpdate,
        message::MaybeInaccessibleMessage,
        reply_markup::{InlineKeyboardButtonBuilder, InlineKeyboardMarkup, ReplyMarkup},
        update::UpdateContent,
    },
    params::{
        self, answer_callback_query::AnswerCallbackQueryParamsBuilder,
        get_updates_params::GetUpdatesParamsBuilder,
    },
    responses::MethodResponse,
    BotCommands, CommandHandler, EventHandler, TelegramError, TelegramResult, UpdateHandler,
};

const TELEGRAM_API_URL: &str = "https://api.telegram.org";

#[must_use]
#[derive(Debug, Clone)]
pub struct Bot {
    token: Arc<str>,
    api_url: Arc<reqwest::Url>,
    client: reqwest::Client,
    handler: EventHandler,
}

impl Bot {
    pub fn new(token: &str) -> Self {
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(17))
            .build()
            .expect("failed to create reqwest::Client");

        let token = Into::<String>::into(token).into();
        let api_url = Arc::new(
            reqwest::Url::parse(TELEGRAM_API_URL)
                .expect("failed to parse default Telegram bot API url"),
        );

        Self {
            token,
            api_url,
            client,
            handler: EventHandler::default(),
        }
    }

    pub fn register_update_handler(&mut self, handler: UpdateHandler) {
        self.handler.register_update_handler(handler);
    }

    pub fn register_commands_handler(
        &mut self,
        commands: impl BotCommands,
        handler: CommandHandler,
    ) {
        self.handler.register_command_handler(commands, handler);
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
                            let result = bot.process_update(&update.content).await;
                            if let Err(e) = result {
                                eprintln!("{}", e);
                            }
                        });
                    }
                }
                None => {
                    eprintln!("No updates found: {:?}", updates_res.description);
                }
            }
        }
    }

    async fn process_update(&self, content: &UpdateContent) -> Result<(), TelegramError> {
        match content {
            UpdateContent::Message(message) => {
                let button = InlineKeyboardButtonBuilder::default()
                    .text("hello")
                    .callback_data("hello callback".to_string())
                    .build()
                    .unwrap();

                let inline_keyboards = vec![button];
                let inline_keyboards_markup = InlineKeyboardMarkup {
                    inline_keyboard: vec![inline_keyboards],
                };
                let inline_keyboards = ReplyMarkup::InlineKeyboardMarkup(inline_keyboards_markup);
                let chat_id = message.chat.id;
                let text = format!("You said: {}", message.text.as_ref().unwrap());
                let param = params::send_message_params::SendMessageParamsBuilder::default()
                    .chat_id(chat_id)
                    .text(text.clone())
                    .reply_markup(inline_keyboards)
                    .build()?;
                let response = self.send_message(&param).await?;
            }
            UpdateContent::CallbackQuery(callback_query) => {
                print!("{:?}", callback_query.message.as_ref().unwrap());
                if let MaybeInaccessibleMessage::Message(message) =
                    callback_query.message.as_ref().unwrap()
                {
                    let chat_id = message.chat.id;
                    let text = format!("You clicked: {}", callback_query.data.as_ref().unwrap());
                    let param = AnswerCallbackQueryParamsBuilder::default()
                        .callback_query_id(callback_query.id.clone())
                        .text(text)
                        .build()?;
                    let response = self.answer_callback_query(&param).await;
                    println!("{:?}", response);
                }
            }
            _ => {
                _ = self.handler.update_handler.as_ref().unwrap()(self.clone(), content.clone());
            }
        }
        Ok(())
    }
}

/// Getters
impl Bot {
    /// Returns currently used token.
    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Returns currently used http-client.
    #[must_use]
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Returns currently used token API url.
    #[must_use]
    pub fn api_url(&self) -> reqwest::Url {
        reqwest::Url::clone(&*self.api_url)
    }
}

impl Bot {
    pub async fn do_request<P, T>(
        &self,
        method: &str,
        params: Option<&P>,
    ) -> Result<MethodResponse<T>, TelegramError>
    where
        P: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
        T: DeserializeOwned + Debug,
    {
        let url = format!("{}bot{}/{}", self.api_url(), self.token(), method);
        let mut prepared_request = self
            .client
            .post(url.clone())
            .header("Content-Type", "application/json");

        prepared_request = if let Some(data) = params {
            let json_string = Self::encode_params(&data)?;
            println!("request url: {:?}, params: {}", url, json_string);
            prepared_request.body(json_string)
        } else {
            prepared_request
        };

        let response_str = prepared_request.send().await?.text().await?;
        println!("response: {}", response_str);
        let response: MethodResponse<T> = serde_json::from_str(&response_str)?;
        Ok(response)
    }

    pub fn encode_params<T: serde::ser::Serialize + std::fmt::Debug>(
        params: &T,
    ) -> Result<String, TelegramError> {
        serde_json::to_string(params).map_err(|e| TelegramError::from(e))
    }
}

#[cfg(test)]
mod tests {
    use crate::{models::user::User, params::send_message_params::SendMessageParams};

    use super::*;

    #[tokio::test]
    async fn test_encode_params() {
        let p = SendMessageParams {
            chat_id: 1132,
            text: "hello".to_string(),
            ..Default::default()
        };
        let encoded = Bot::encode_params(&p).unwrap();
        println!("{}", encoded);
    }

    #[tokio::test]
    async fn test_do_request() {
        let bot = Bot::new("6616659571:AAEr0TdwPXBnvHQl_VJj5Z6wh-p3uUDNbOw");
        let result = bot.do_request::<String, User>("getMe", None).await;
        assert!(result.is_ok());
    }
}
