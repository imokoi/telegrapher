use reqwest::{
    multipart::{self, Part},
    Client,
};
use serde_json::Value;

use crate::{
    bot::Bot,
    models::{message::Message, sticker::FileUpload},
    params::media_params::SendPhotoParams,
    requests,
    responses::MethodResponse,
    TelegrapherError,
};

impl Bot {
    pub async fn send_photo(
        &self,
        params: &SendPhotoParams,
    ) -> Result<MethodResponse<Message>, TelegrapherError> {
        if let FileUpload::String(_) = &params.photo {
            return requests::post_request::<SendPhotoParams, Message>(
                "sendPhoto",
                self.token(),
                Some(params),
            )
            .await;
        }

        let input_file = match &params.photo {
            FileUpload::InputFile(path) => path,
            _ => return Err(TelegrapherError::from("Invalid file path")),
        };
        requests::post_multi_part_request::<SendPhotoParams, Message>(
            "sendPhoto",
            self.token(),
            Some(params),
            &input_file.path,
            &requests::FileType::Photo,
        )
        .await
    }
}
