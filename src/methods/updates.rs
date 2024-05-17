use crate::params::get_updates::GetUpdatesParams;
use crate::{bot::Bot, models::update::Update, TelegramError};

pub async fn get_updates(
    bot: &Bot,
    offset: Option<i64>,
    limit: Option<u32>,
    timeout: Option<u32>,
) -> Result<Vec<Update>, TelegramError> {
    let get_updates_params = GetUpdatesParams {
        offset,
        limit,
        timeout,
        allowed_updates: None,
    };
    bot.do_request::<GetUpdatesParams, Vec<Update>>("getUpdates", Some(&get_updates_params))
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_updates() {
        let bot = Bot::new("6616659571:AAEr0TdwPXBnvHQl_VJj5Z6wh-p3uUDNbOw");
        let updates = get_updates(&bot, None, None, None).await.unwrap();
        println!("{:?}", updates);
    }
}
