use super::{message::MaybeInaccessibleMessage, user::User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<MaybeInaccessibleMessage>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}
