use crate::{
    bot::Bot,
    models::message::Message,
    params::{callback_query_param::AnswerCallbackQueryParams, message_params::SendMessageParams},
    requests,
    responses::MethodResponse,
    TelegramError,
};

impl Bot {
    /// Send a message to a chat.
    /// [The official docs](https://core.telegram.org/bots/api#sendmessage)
    pub async fn send_message(
        &self,
        params: &SendMessageParams,
    ) -> Result<MethodResponse<Message>, TelegramError> {
        requests::post_request::<SendMessageParams, Message>(
            "sendMessage",
            self.token(),
            Some(params),
        )
        .await
    }

    // async fn send_photo()

    pub async fn answer_callback_query(
        &self,
        params: &AnswerCallbackQueryParams,
    ) -> Result<MethodResponse<bool>, TelegramError> {
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
    use super::*;
    use crate::{
        models::{
            parse_mode::ParseMode,
            reply_markup::{InlineKeyboardButtonBuilder, InlineKeyboardMarkup, ReplyMarkup},
        },
        params::message_params::SendMessageParamsBuilder,
    };

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

        let message = bot.send_message(&params).await.unwrap();
        println!("{:?}", message);
    }
}
