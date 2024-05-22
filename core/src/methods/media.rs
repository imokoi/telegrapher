use reqwest::{
    multipart::{self, Part},
    Client,
};
use serde_json::Value;

use crate::{
    bot::Bot,
    models::{message::Message, sticker::FileUpload},
    params::media_params::SendPhotoParams,
    responses::MethodResponse,
    TelegramError,
};

impl Bot {
    pub async fn send_photo(
        &self,
        params: &SendPhotoParams,
    ) -> Result<MethodResponse<Message>, TelegramError> {
        let url = format!("{}bot{}/{}", self.api_url(), self.token(), "sendPhoto");
        if let FileUpload::String(_) = &params.photo {
            return self
                .do_request::<SendPhotoParams, Message>("sendPhoto", Some(params))
                .await;
        }

        let json_string = Self::encode_params(&params)?;
        let json_struct: Value = serde_json::from_str(&json_string).unwrap();

        let input_file = match &params.photo {
            FileUpload::InputFile(path) => path,
            _ => return Err(TelegramError::from("Invalid file path")),
        };
        let filename = input_file.path.file_name().ok_or_else(|| {
            TelegramError::from("Photo path is not valid. Please provide a valid path.")
        })?;
        let filename_str = filename.to_str().ok_or_else(|| {
            TelegramError::from("Photo path is not valid. Please provide a valid path.")
        })?;
        let mut form = multipart::Form::new();
        for (key, val) in json_struct.as_object().unwrap() {
            if !key.as_str().eq("photo") {
                let val = match val {
                    Value::String(val) => val.to_string(),
                    other => other.to_string(),
                };

                form = form.text(key.clone(), val);
            }
        }
        form = form.part(
            "photo",
            Part::stream(reqwest::Body::from(std::fs::read(&input_file.path)?))
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
