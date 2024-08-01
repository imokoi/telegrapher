use crate::params::media_params::SendDocumentParams;
use crate::{
    bot::Bot,
    models::{message::Message, sticker::FileUpload},
    params::media_params::SendPhotoParams,
    requests,
    responses::MethodResponse,
    FileType, TelegrapherError,
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
            &FileType::Photo,
        )
        .await
    }

    pub async fn send_document(
        &self,
        params: &SendDocumentParams,
    ) -> Result<MethodResponse<Message>, TelegrapherError> {
        if let FileUpload::String(_) = &params.document {
            return requests::post_request::<SendDocumentParams, Message>(
                "sendDocument",
                self.token(),
                Some(params),
            )
            .await;
        }

        let input_file = match &params.document {
            FileUpload::InputFile(path) => path,
            _ => return Err(TelegrapherError::from("Invalid file path")),
        };
        requests::post_multi_part_request::<SendDocumentParams, Message>(
            "sendDocument",
            self.token(),
            Some(params),
            &input_file.path,
            &FileType::Document,
        )
        .await
    }
}
