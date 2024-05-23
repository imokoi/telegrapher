use crate::{
    bot::Bot,
    models::webhook::WebhookInfo,
    params::webhook_param::{DeleteWebhookParams, SetWebhookParams},
    requests,
    responses::MethodResponse,
    TelegrapherError,
};

impl Bot {
    /// Set a webhook for the bot.
    pub async fn set_webhook(
        &self,
        params: &SetWebhookParams,
    ) -> Result<MethodResponse<bool>, TelegrapherError> {
        requests::post_request::<SetWebhookParams, bool>("setWebhook", self.token(), Some(params))
            .await
    }

    /// Delete the webhook for the bot.
    pub async fn delete_webhook(
        &self,
        params: &DeleteWebhookParams,
    ) -> Result<MethodResponse<bool>, TelegrapherError> {
        requests::post_request::<DeleteWebhookParams, bool>(
            "deleteWebhook",
            self.token(),
            Some(params),
        )
        .await
    }

    /// Get the webhook info for the bot.
    pub async fn get_webhook_info(&self) -> Result<MethodResponse<WebhookInfo>, TelegrapherError> {
        requests::post_request::<(), WebhookInfo>("getWebhookInfo", self.token(), None).await
    }
}
