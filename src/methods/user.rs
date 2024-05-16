use crate::{bot::Bot, models::User, TelegramError};

pub async fn get_me(bot: &Bot) -> Result<User, TelegramError> {
    bot.do_request::<String, User>("getMe", None).await
}

pub async fn log_out(bot: &Bot) -> Result<bool, TelegramError> {
    bot.do_request::<String, bool>("logOut", None).await
}

pub async fn close(bot: &Bot) -> Result<bool, TelegramError> {
    bot.do_request::<String, bool>("close", None).await
}
