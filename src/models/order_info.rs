use serde::{Deserialize, Serialize};

use crate::models::shipping_query::ShippingAddress;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>,
}
