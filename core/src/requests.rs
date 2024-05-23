use crate::{responses::MethodResponse, TelegrapherError, TELEGRAM_API_URL};
use reqwest::multipart::{self, Part};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{fmt::Debug, path::PathBuf, time::Duration};

/// post a normal http request to the telegram api
pub async fn post_request<P, T>(
    method: &str,
    token: &str,
    params: Option<&P>,
) -> Result<MethodResponse<T>, TelegrapherError>
where
    P: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
    T: DeserializeOwned + Debug,
{
    let api_base_url = reqwest::Url::parse(TELEGRAM_API_URL)
        .expect("failed to parse default Telegram bot API url");
    let url = format!("{}bot{}/{}", api_base_url, token, method);
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(17))
        .build()
        .expect("failed to create reqwest::Client");

    let mut prepared_request = client
        .post(url.clone())
        .header("Content-Type", "application/json");

    prepared_request = if let Some(data) = params {
        let json_string = encode_params(&data)?;
        println!("json string: {}", json_string);
        prepared_request.body(json_string)
    } else {
        prepared_request
    };

    let response_str = prepared_request.send().await?.text().await?;
    let response: MethodResponse<T> = serde_json::from_str(&response_str)?;
    Ok(response)
}

pub async fn post_multi_part_request<P, T>(
    method: &str,
    token: &str,
    params: Option<&P>,
    file_path: &PathBuf,
    file_type: &FileType,
) -> Result<MethodResponse<T>, TelegrapherError>
where
    P: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
    T: DeserializeOwned + Debug,
{
    let api_base_url = reqwest::Url::parse(TELEGRAM_API_URL)
        .expect("failed to parse default Telegram bot API url");
    let url = format!("{}bot{}/{}", api_base_url, token, method);
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(17))
        .build()
        .expect("failed to create reqwest::Client");

    let json_string = encode_params(&params)?;
    let json_struct: Value = serde_json::from_str(&json_string).unwrap();
    let filename = file_path.file_name().ok_or_else(|| {
        TelegrapherError::from("file path is not valid. please provide a valid path.")
    })?;
    let filename_str = filename.to_str().ok_or_else(|| {
        TelegrapherError::from("file path is not valid. Please provide a valid path.")
    })?;
    let mut form = multipart::Form::new();
    for (key, val) in json_struct.as_object().unwrap() {
        if !key.as_str().eq(file_type.to_string().as_str()) {
            let val = match val {
                Value::String(val) => val.to_string(),
                other => other.to_string(),
            };

            form = form.text(key.clone(), val);
        }
    }
    form = form.part(
        file_type.to_string(),
        Part::stream(reqwest::Body::from(std::fs::read(file_path)?))
            .file_name(filename_str.to_string()),
    );
    let res = client
        .post(&url)
        .multipart(form)
        .send()
        .await?
        .text()
        .await?;
    let response: MethodResponse<T> = serde_json::from_str(&res)?;
    Ok(response)
}

pub fn encode_params<T: serde::ser::Serialize + std::fmt::Debug>(
    params: &T,
) -> Result<String, TelegrapherError> {
    serde_json::to_string(params).map_err(|e| TelegrapherError::from(e))
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

impl ToString for FileType {
    fn to_string(&self) -> String {
        match self {
            FileType::Photo => "photo".to_string(),
            FileType::Video => "video".to_string(),
            FileType::Document => "document".to_string(),
            FileType::Audio => "audio".to_string(),
            FileType::Voice => "voice".to_string(),
            FileType::Animation => "animation".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bot::Bot, models::user::User, params::message_params::SendMessageParams};

    use super::*;

    #[tokio::test]
    async fn test_encode_params() {
        let p = SendMessageParams {
            chat_id: 1132,
            text: "hello".to_string(),
            ..Default::default()
        };
        let encoded = encode_params(&p).unwrap();
        println!("{}", encoded);
    }

    #[tokio::test]
    async fn test_do_request() {
        let bot = Bot::new("6616659571:AAEr0TdwPXBnvHQl_VJj5Z6wh-p3uUDNbOw");
        let result = post_request::<String, User>("getMe", bot.token(), None).await;
        assert!(result.is_ok());
    }
}
