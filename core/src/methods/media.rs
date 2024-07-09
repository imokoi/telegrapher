use crate::{
    bot::Bot,
    FileType,
    models::{message::Message, sticker::FileUpload},
    params::media_params::SendPhotoParams,
    requests,
    responses::MethodResponse, TelegrapherError,
};
use crate::params::media_params::SendDocumentParams;

impl Bot {
    pub async fn send_photo(
        &self,
        params: &SendPhotoParams,
    ) -> Result<MethodResponse<Message>, TelegrapherError> {
        if let Ok(_) = self.get_message_send_permissions(params.chat_id).await {
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
        } else {
            Err(TelegrapherError::from(
                "failed to get permission to send message to the chat",
            ))
        }
    }

    pub async fn send_document(
        &self,
        params: &SendDocumentParams,
    ) -> Result<MethodResponse<Message>, TelegrapherError> {
        if let Ok(_) = self.get_message_send_permissions(params.chat_id).await {
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
        } else {
            Err(TelegrapherError::from(
                "failed to get permission to send message to the chat",
            ))
        }
    }
}
