use crate::params::updates_params::GetUpdatesParams;
use crate::requests;
use crate::responses::MethodResponse;
use crate::{bot::Bot, models::update::Update, TelegrapherError};

impl Bot {
    pub async fn get_updates(
        &self,
        params: &GetUpdatesParams,
    ) -> Result<MethodResponse<Vec<Update>>, TelegrapherError> {
        requests::post_request::<GetUpdatesParams, Vec<Update>>(
            "getUpdates",
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
        models::allowed_update::AllowedUpdate, params::updates_params::GetUpdatesParamsBuilder,
    };

    #[tokio::test]
    async fn test_get_updates() {
        let bot = Bot::new("6616659571:AAEr0TdwPXBnvHQl_VJj5Z6wh-p3uUDNbOw", 1);
        let get_updates_params = GetUpdatesParamsBuilder::default()
            .allowed_updates(vec![AllowedUpdate::Message, AllowedUpdate::CallbackQuery])
            .build()
            .unwrap();
        let updates = bot.get_updates(&get_updates_params).await.unwrap();
        println!("{:?}", updates);
    }
}
