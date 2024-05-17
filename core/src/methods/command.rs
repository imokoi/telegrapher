use crate::{
    bot::Bot,
    models::command::BotCommand,
    params::command_params::{DeleteMyCommandsParams, GetMyCommandsParams, SetMyCommandsParams},
    responses::MethodResponse,
    TelegramError,
};

impl Bot {
    pub async fn set_my_commands(
        &self,
        params: &SetMyCommandsParams,
    ) -> Result<MethodResponse<bool>, TelegramError> {
        self.do_request::<SetMyCommandsParams, bool>("setMyCommands", Some(params))
            .await
    }

    pub async fn delete_my_commands(
        &self,
        params: &DeleteMyCommandsParams,
    ) -> Result<MethodResponse<bool>, TelegramError> {
        self.do_request::<DeleteMyCommandsParams, bool>("deleteMyCommands", Some(params))
            .await
    }

    pub async fn get_my_commands(
        &self,
        params: &GetMyCommandsParams,
    ) -> Result<MethodResponse<Vec<BotCommand>>, TelegramError> {
        self.do_request::<GetMyCommandsParams, Vec<BotCommand>>("getMyCommands", Some(params))
            .await
    }
}
