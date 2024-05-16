use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageReactionUpdated {
    pub chat: Chat,
    pub message_id: i32,
    pub user: Option<User>,
    pub actor_chat: Option<Chat>,
    pub date: u64,
    pub old_reaction: Vec<ReactionType>,
    pub new_reaction: Vec<ReactionType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReactionType {
    Emoji(ReactionTypeEmoji),
    CustomEmoji(ReactionTypeCustomEmoji),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReactionTypeEmoji {
    pub emoji: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReactionTypeCustomEmoji {
    pub custom_emoji_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReactionCount {
    #[serde(rename = "type")]
    pub type_field: ReactionType,

    pub total_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageReactionCountUpdated {
    pub chat: Chat,
    pub message_id: i32,
    pub date: u64,
    pub reactions: Vec<ReactionCount>,
}
