pub mod bot;
pub mod methods;
pub mod models;
pub mod requests;
pub mod responses;

use async_trait::async_trait;

pub type TelegramError = Box<dyn std::error::Error + Send + Sync>;

#[async_trait]
pub trait TelegramApi {
    async fn get_me(&self) -> Result<String, TelegramError> {
        Ok("get_me".to_string())
    }
}

fn test() -> String {
    return "hello".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        assert_eq!(test(), "hello");
    }
}
