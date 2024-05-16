use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Giveaway {
    pub chats: Vec<Chat>,
    pub winners_selection_date: u64,
    pub winner_count: u32,
    pub only_new_members: Option<bool>,
    pub has_public_winners: Option<bool>,
    pub prize_description: Option<String>,
    pub country_codes: Option<Vec<String>>,
    pub premium_subscription_month_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GiveawayCreated {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayWinners {
    pub chat: Chat,
    pub giveaway_message_id: i32,
    pub winners_selection_date: u64,
    pub winner_count: u32,
    pub winners: Vec<User>,
    pub additional_chat_count: Option<u32>,
    pub premium_subscription_month_count: Option<u32>,
    pub unclaimed_prize_count: Option<u32>,
    pub only_new_members: Option<bool>,
    pub was_refunded: Option<bool>,
    pub prize_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayCompleted {
    pub winner_count: u32,
    pub unclaimed_prize_count: Option<u32>,
    pub giveaway_message: Option<Box<Message>>,
}
