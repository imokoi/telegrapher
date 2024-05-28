use std::{arch::global_asm, time::Duration};

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
        if params.chat_id > 0 {
            let rate_limiter = self.rate_limiter.clone();
            let user_chat_sem = rate_limiter.acquire_user_chat(params.chat_id).await;
            let user_chat_permit = user_chat_sem.acquire_owned().await;
            if user_chat_permit.is_err() {
                return Err("User chat semaphore error".into());
            }

            let global_chat_sem = rate_limiter.acquire_global().await;
            let global_chat_permit = global_chat_sem.acquire_owned().await;
            if global_chat_permit.is_err() {
                return Err("Global chat semaphore error".into());
            }

            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(1000 / 30)).await;
                drop(global_chat_permit);

                tokio::time::sleep(Duration::from_secs_f32(1.0)).await;
                drop(user_chat_permit);
            });
        } else {
            let rate_limiter = self.rate_limiter.clone();
            let group_chat_sem = rate_limiter.acquire_user_chat(params.chat_id).await;
            let group_chat_permit = group_chat_sem.acquire_owned().await;
            if group_chat_permit.is_err() {
                return Err("Group chat semaphore error".into());
            }

            let global_chat_sem = rate_limiter.acquire_global().await.clone();
            let global_chat_permit = global_chat_sem.acquire_owned().await;
            if global_chat_permit.is_err() {
                return Err("Global chat semaphore error".into());
            }

            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(1000 / 30)).await;
                drop(global_chat_permit);

                tokio::time::sleep(Duration::from_secs(60 / 20)).await;
                drop(group_chat_permit);
            });
        }
        requests::post_request::<SendMessageParams, Message>(
            "sendMessage",
            self.token(),
            Some(params),
        )
        .await
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

        let message = bot.send_message_throttled(&params).await.unwrap();
        println!("{:?}", message);
    }
}
