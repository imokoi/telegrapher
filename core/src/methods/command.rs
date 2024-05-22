use crate::{
    bot::Bot,
    models::command::BotCommand,
    params::command_params::{DeleteMyCommandsParams, GetMyCommandsParams, SetMyCommandsParams},
    requests,
    responses::MethodResponse,
    TelegramError,
};

impl Bot {
    pub async fn set_my_commands(
        &self,
        params: &SetMyCommandsParams,
    ) -> Result<MethodResponse<bool>, TelegramError> {
        requests::post_request::<SetMyCommandsParams, bool>(
            "setMyCommands",
            self.token(),
            Some(params),
        )
        .await
    }

    pub async fn delete_my_commands(
        &self,
        params: &DeleteMyCommandsParams,
    ) -> Result<MethodResponse<bool>, TelegramError> {
        requests::post_request::<DeleteMyCommandsParams, bool>(
            "deleteMyCommands",
            self.token(),
            Some(params),
        )
        .await
    }

    pub async fn get_my_commands(
        &self,
        params: &GetMyCommandsParams,
    ) -> Result<MethodResponse<Vec<BotCommand>>, TelegramError> {
        requests::post_request::<GetMyCommandsParams, Vec<BotCommand>>(
            "getMyCommands",
            self.token(),
            Some(params),
        )
        .await
    }
}
