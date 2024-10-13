use serde::{Deserialize, Serialize};

use crate::models::animation::Animation;
use crate::models::message_entity::MessageEntity;
use crate::models::photo_size::PhotoSize;
use crate::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameHighScore {
    pub position: u64,
    pub user: User,
    pub score: i64,
}
