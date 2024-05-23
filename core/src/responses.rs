use serde::{Deserialize, Serialize};

use crate::{JsonData, TelegrapherResult};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MethodResponse<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub error_code: Option<u64>,
    pub description: Option<String>,
    pub parameters: Option<ResponseParameters>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ResponseParameters {
    pub migrate_to_chat_id: Option<i64>,
    pub retry_after: Option<u16>,
}

pub fn build_webhook_response<T: Serialize>(
    method: &str,
    params: T,
) -> TelegrapherResult<JsonData> {
    let mut json = serde_json::to_value(params)?;
    let json_object = json.as_object_mut().ok_or("Not an object")?;
    json_object.insert(
        "method".to_string(),
        serde_json::Value::String(method.to_string()),
    );
    Ok(json)
}

#[cfg(test)]
mod tests {
    use crate::params::message_params::SendMessageParamsBuilder;

    use super::*;

    #[test]
    fn test_build_webhook_response() {
        let method = "sendMessage";
        let params = SendMessageParamsBuilder::default()
            .text("Hello")
            .chat_id(123456)
            .build()
            .unwrap();
        let result = build_webhook_response(method, params);
        print!("{:?}", result);
    }
}
