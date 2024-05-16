use serde::{Deserialize, Serialize};

use crate::models::order_info::OrderInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: u32,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}
