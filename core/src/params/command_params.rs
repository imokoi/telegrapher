use crate::models::command::BotCommand;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into))]
pub struct SetMyCommandsParams {
    commands: Vec<BotCommand>,
    scope: BotCommandScope,
    language_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteMyCommandsParams {
    scope: BotCommandScope,
    language_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetMyCommandsParams {
    scope: BotCommandScope,
    language_code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BotCommandScope {
    Default,
    AllPrivateChats,
    AllGroupChats,
    AllChatAdministrators,
    Chat(BotCommandScopeChat),
    ChatAdministrators(BotCommandScopeChatAdministrators),
    ChatMember(BotCommandScopeChatMember),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BotCommandScopeChat {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BotCommandScopeChatAdministrators {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BotCommandScopeChatMember {
    pub chat_id: ChatId,
    pub user_id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ChatId {
    Integer(i64),
    String(String),
}

impl From<i64> for ChatId {
    fn from(id: i64) -> Self {
        Self::Integer(id)
    }
}

impl From<String> for ChatId {
    fn from(id: String) -> Self {
        Self::String(id)
    }
}
