use reqwest::{
    multipart::{self, Part},
    Client,
};

use crate::{bot::Bot, models::message::Message, responses::MethodResponse, TelegramError};
use std::path::{self, Path};

impl Bot {
    pub async fn send_photo(
        &self,
        chat_id: &str,
        photo_path: &Path,
    ) -> Result<MethodResponse<Message>, TelegramError> {
        let url = format!("{}bot{}/{}", self.api_url(), self.token(), "sendPhoto");
        let filename = photo_path.file_name().ok_or_else(|| {
            TelegramError::from("Photo path is not valid. Please provide a valid path.")
        })?;
        let filename_str = filename.to_str().ok_or_else(|| {
            TelegramError::from("Photo path is not valid. Please provide a valid path.")
        })?;
        let form = multipart::Form::new()
            .text("chat_id", chat_id.to_string())
            .part(
                "photo",
                Part::stream(reqwest::Body::from(std::fs::read(photo_path)?))
                    .file_name(filename_str.to_string()),
            );
        let client = Client::new();
        let res = client
            .post(&url)
            .multipart(form)
            .send()
            .await?
            .text()
            .await?;
        let response: MethodResponse<Message> = serde_json::from_str(&res)?;
        Ok(response)
    }
}
