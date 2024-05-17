use std::{fmt::Debug, sync::Arc, time::Duration};

use async_trait::async_trait;
use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::{
    methods,
    models::{message::Message, update::UpdateContent},
    responses::MethodResponse,
    TelegramApi, TelegramError,
};

const TELEGRAM_API_URL: &str = "https://api.telegram.org";

#[must_use]
#[derive(Debug, Clone)]
pub struct Bot {
    token: Arc<str>,
    api_url: Arc<reqwest::Url>,
    client: reqwest::Client,
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
        }
    }

    /// Start getting updates from telegram api server.
    pub async fn start(&self) -> Result<(), TelegramError> {
        let mut offset = 0;
        loop {
            let updates = methods::updates::get_updates(self, Some(offset), None, Some(60)).await;
            match updates {
                Ok(mut updates) => {
                    // Sort updates by update_id
                    updates.sort_by(|a, b| a.update_id.cmp(&b.update_id));
                    for update in updates {
                        offset = update.update_id + 1;
                        let bot = Arc::new(self.clone());
                        if let UpdateContent::Message(message) = update.content {
                            tokio::spawn(async move {
                                bot.process_message(message).await;
                            });
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }

            tokio::time::sleep(Duration::from_secs_f32(0.2)).await;
        }
    }

    async fn process_message(&self, message: Message) {
        let chat_id = message.chat.id;
        let text = message.text.unwrap_or_else(|| "".to_string());
        let response = methods::message::send_message(self, chat_id, text, None).await;
        match response {
            Ok(_) => {
                println!("Message sent successfully");
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
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
    ) -> Result<T, TelegramError>
    where
        P: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
        T: DeserializeOwned + Debug,
    {
        let url = format!("{}bot{}/{}?", self.api_url(), self.token(), method);
        let mut prepared_request = self
            .client
            .post(url)
            .header("Content-Type", "application/json");

        prepared_request = if let Some(data) = params {
            let json_string = Self::encode_params(&data)?;
            println!("{}", json_string);
            prepared_request.body(json_string)
        } else {
            prepared_request
        };

        let response_str = prepared_request.send().await?.text().await?;
        let response: MethodResponse<T> = serde_json::from_str(&response_str)?;
        Ok(response.result)
    }

    pub fn encode_params<T: serde::ser::Serialize + std::fmt::Debug>(
        params: &T,
    ) -> Result<String, TelegramError> {
        serde_json::to_string(params).map_err(|e| TelegramError::from(e))
    }
}

#[async_trait]
impl TelegramApi for Bot {}

#[cfg(test)]
mod tests {
    use crate::{models::user::User, params::send_message::SendMessageParams};

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
