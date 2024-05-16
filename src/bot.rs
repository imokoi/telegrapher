use crate::{responses::MethodResponse, TelegramApi, TelegramError};
use async_trait::async_trait;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::{fmt::Debug, sync::Arc, time::Duration};

const TELEGRAM_API_URL: &str = "https://api.telegram.org";

#[must_use]
#[derive(Debug, Clone)]
pub struct Bot {
    token: Arc<str>,
    api_url: Arc<reqwest::Url>,
    client: reqwest::Client,
}

impl Bot {
    #[must_use]
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
