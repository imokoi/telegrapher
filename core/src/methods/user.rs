use crate::{bot::Bot, models::user::User, requests, responses::MethodResponse, TelegrapherError};

impl Bot {
    pub async fn get_me(&self) -> Result<MethodResponse<User>, TelegrapherError> {
        requests::post_request::<(), User>("getMe", self.token(), None).await
    }

    pub async fn log_out(&self) -> Result<MethodResponse<bool>, TelegrapherError> {
        requests::post_request::<(), bool>("logOut", self.token(), None).await
    }

    pub async fn close(&self) -> Result<MethodResponse<bool>, TelegrapherError> {
        requests::post_request::<(), bool>("close", self.token(), None).await
    }
}
