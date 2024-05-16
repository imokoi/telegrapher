use serde::{Deserialize, Serialize};

/// Represents an incoming update from telegram.
/// [Official documentation.](https://core.telegram.org/bots/api#update)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Update {
    pub update_id: u32,

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
