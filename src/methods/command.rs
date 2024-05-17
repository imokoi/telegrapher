use crate::{
    bot::Bot,
    models::command::BotCommand,
    params::command_params::{DeleteMyCommandsParams, GetMyCommandsParams, SetMyCommandsParams},
    TelegramError,
};

impl Bot {
    pub async fn set_my_commands(
        &self,
        params: &SetMyCommandsParams,
    ) -> Result<bool, TelegramError> {
        self.do_request::<SetMyCommandsParams, bool>("setMyCommands", Some(params))
            .await
    }

    pub async fn delete_my_commands(
        &self,
        params: &DeleteMyCommandsParams,
    ) -> Result<bool, TelegramError> {
        self.do_request::<DeleteMyCommandsParams, bool>("deleteMyCommands", Some(params))
            .await
    }

    pub async fn get_my_commands(
        &self,
        params: &GetMyCommandsParams,
    ) -> Result<Vec<BotCommand>, TelegramError> {
        self.do_request::<GetMyCommandsParams, Vec<BotCommand>>("getMyCommands", Some(params))
            .await
    }
}
