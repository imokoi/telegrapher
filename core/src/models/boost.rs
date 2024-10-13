use super::{chat::Chat, user::User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatBoostAdded {
    pub boost_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum ChatBoostSource {
    Premium(ChatBoostSourcePremium),
    GiftCode(ChatBoostSourceGiftCode),
    Giveaway(ChatBoostSourceGiveaway),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatBoostSourcePremium {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatBoostSourceGiftCode {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatBoostSourceGiveaway {
    pub giveaway_message_id: i64,
    pub user: Option<User>,
    pub is_unclaimed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatBoost {
    pub boost_id: String,
    pub add_date: u64,
    pub expiration_date: u64,
    pub source: ChatBoostSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostUpdated {
    pub chat: Chat,
    pub boost: ChatBoost,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostRemoved {
    pub chat: Chat,
    pub boost_id: String,
    pub remove_date: u64,
    pub source: ChatBoostSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserChatBoosts {
    pub boosts: Vec<ChatBoost>,
}
