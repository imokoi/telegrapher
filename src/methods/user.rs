use crate::{bot::Bot, models::user::User, TelegramError};

impl Bot {
    pub async fn get_me(&self) -> Result<User, TelegramError> {
        self.do_request::<(), User>("getMe", None).await
    }

    pub async fn log_out(&self) -> Result<bool, TelegramError> {
        self.do_request::<(), bool>("logOut", None).await
    }

    pub async fn close(&self) -> Result<bool, TelegramError> {
        self.do_request::<(), bool>("close", None).await
    }
}
