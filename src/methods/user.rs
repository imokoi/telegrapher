use crate::{bot::Bot, models::user::User, TelegramError};

pub async fn get_me(bot: &Bot) -> Result<User, TelegramError> {
    bot.do_request::<(), User>("getMe", None).await
}

pub async fn log_out(bot: &Bot) -> Result<bool, TelegramError> {
    bot.do_request::<(), bool>("logOut", None).await
}

pub async fn close(bot: &Bot) -> Result<bool, TelegramError> {
    bot.do_request::<(), bool>("close", None).await
}
