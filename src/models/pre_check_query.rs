use serde::{Deserialize, Serialize};

use crate::models::order_info::OrderInfo;
use crate::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: u32,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
}
