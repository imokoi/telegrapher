use crate::{
    bot::Bot,
    models::message::Message,
    params::{
        callback_query_param::AnswerCallbackQueryParams,
        message_params::{DeleteMessageParams, EditMessageTextParams, SendMessageParams},
    },
    requests,
    responses::MethodResponse,
    TelegrapherError,
};

impl Bot {
    /// Send a message to the message channel. this method will retry with retry_after time automatically.
    pub async fn send_message_throttled_and_retry(&self, params: &SendMessageParams) {
        let message_sender;
        {
            message_sender = self.message_sender.lock().await;
        }
        if let Some(sender) = message_sender.as_ref() {
            _ = sender.send(params.clone()).await;
        }
    }

    /// Send a message to a chat. this method will send message slowly to avoid being banned by telegram.
    /// [The official docs](https://core.telegram.org/bots/api#sendmessage)
    pub async fn send_message_throttled(
        &self,
        params: &SendMessageParams,
    ) -> Result<MethodResponse<Message>, TelegrapherError> {
        if let Ok(_) = self.get_message_send_permissions(params.chat_id).await {
            requests::post_request::<SendMessageParams, Message>(
                "sendMessage",
                self.token(),
                Some(params),
            )
            .await
        } else {
            Err(TelegrapherError::from(
                "faild to get permission to send message to the chat",
            ))
        }
    }

    pub async fn edit_message(
        &self,
        params: &EditMessageTextParams,
    ) -> Result<MethodResponse<Message>, TelegrapherError> {
        requests::post_request::<EditMessageTextParams, Message>(
            "editMessageText",
            self.token(),
            Some(params),
        )
        .await
    }

    pub async fn delete_message(
        &self,
        params: &DeleteMessageParams,
    ) -> Result<MethodResponse<bool>, TelegrapherError> {
        requests::post_request::<DeleteMessageParams, bool>(
            "deleteMessage",
            self.token(),
            Some(&params),
        )
        .await
    }

    pub async fn answer_callback_query(
        &self,
        params: &AnswerCallbackQueryParams,
    ) -> Result<MethodResponse<bool>, TelegrapherError> {
        requests::post_request::<AnswerCallbackQueryParams, bool>(
            "answerCallbackQuery",
            self.token(),
            Some(params),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{
            parse_mode::ParseMode,
            reply_markup::{InlineKeyboardButtonBuilder, InlineKeyboardMarkup, ReplyMarkup},
        },
        params::message_params::SendMessageParamsBuilder,
    };

    use super::*;

    #[tokio::test]
    async fn test_send_message() {
        let bot = Bot::new("6616659571:AAEr0TdwPXBnvHQl_VJj5Z6wh-p3uUDNbOw");
        let raw_str = r#"
        *bold text*
        "#;

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
        let params = SendMessageParamsBuilder::default()
            .chat_id(1393242628)
            .text(raw_str.to_string())
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(inline_keyboards)
            .build()
            .unwrap();

        let message = bot.send_message_throttled(&params).await.unwrap();
        println!("{:?}", message);
    }
}
