pub mod bot;
pub mod methods;
pub mod models;
pub mod params;
pub mod requests;
pub mod responses;

pub type TelegramError = Box<dyn std::error::Error + Send + Sync>;
