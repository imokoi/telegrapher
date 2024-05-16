use crate::{
    bot::Bot,
    models::{message::Message, parse_mode::ParseMode},
    params::send_message::SendMessageParams,
    TelegramError,
};

/// Send a message to a chat.
/// [The official docs](https://core.telegram.org/bots/api#sendmessage)
async fn send_message(
    bot: &Bot,
    chat_id: i64,
    text: String,
    parse_mode: Option<ParseMode>,
) -> Result<Message, TelegramError> {
    let params = SendMessageParams {
        chat_id,
        text,
        parse_mode,
        ..Default::default()
    };
    bot.do_request::<SendMessageParams, Message>("sendMessage", Some(&params))
        .await
}

// async fn send_photo()

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_message() {
        let bot = Bot::new("6616659571:AAEr0TdwPXBnvHQl_VJj5Z6wh-p3uUDNbOw");
        let raw_str = r#"
Hello, world!
This is a raw string.
Special characters like \n and \t are not escaped.
        "#;

        let message = send_message(&bot, 1393242628, raw_str.to_string(), None)
            .await
            .unwrap();
        println!("{:?}", message);
    }
}
