use crate::models::command::BotCommand;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into))]
pub struct SetMyCommandsParams {
    commands: Vec<BotCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct DeleteMyCommandsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
pub struct GetMyCommandsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    language_code: Option<String>,
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
    pub chat_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BotCommandScopeChatAdministrators {
    pub chat_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BotCommandScopeChatMember {
    pub chat_id: i64,
    pub user_id: u64,
}
