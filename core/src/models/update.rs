use serde::{Deserialize, Serialize};

use crate::models::boost::{ChatBoost, ChatBoostRemoved};
use crate::models::business::{BusinessConnection, BusinessMessagesDeleted};
use crate::models::callback_query::CallbackQuery;
use crate::models::chat::ChatJoinRequest;
use crate::models::chat_member::ChatMemberUpdated;
use crate::models::chosen_inline_result::ChosenInlineResult;
use crate::models::inline_query::InlineQuery;
use crate::models::message::Message;
use crate::models::poll::{Poll, PollAnswer};
use crate::models::pre_check_query::PreCheckoutQuery;
use crate::models::reaction::{MessageReactionCountUpdated, MessageReactionUpdated};
use crate::models::shipping_query::ShippingQuery;

/// Represents an incoming update from telegram.
/// [Official documentation.](https://core.telegram.org/bots/api#update)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Update {
    pub update_id: i64,

    /// Maps to exactly one of the many optional fields
    /// from [the official documentation](https://core.telegram.org/bots/api#update).
    #[serde(flatten)]
    pub content: UpdateContent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateContent {
    Message(Message),
    EditedMessage(Message),
    ChannelPost(Message),
    EditedChannelPost(Message),
    BusinessConnection(BusinessConnection),
    BusinessMessage(Message),
    EditedBusinessMessage(Message),
    DeletedBusinessMessages(BusinessMessagesDeleted),
    MessageReaction(MessageReactionUpdated),
    MessageReactionCount(MessageReactionCountUpdated),
    InlineQuery(InlineQuery),
    ChosenInlineResult(ChosenInlineResult),
    CallbackQuery(CallbackQuery),
    ShippingQuery(ShippingQuery),
    PreCheckoutQuery(PreCheckoutQuery),
    Poll(Poll),
    PollAnswer(PollAnswer),
    MyChatMember(ChatMemberUpdated),
    ChatMember(ChatMemberUpdated),
    ChatJoinRequest(ChatJoinRequest),
    ChatBoost(ChatBoost),
    RemovedChatBoost(ChatBoostRemoved),
}
