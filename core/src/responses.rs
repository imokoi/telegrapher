use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MethodResponse<T> {
    /// Always true
    pub ok: bool,
    pub result: Option<T>,
    pub error_code: Option<u64>,
    pub description: Option<String>,
}

// #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
// /// \[…\] an unsuccessful request, `ok` equals false and the error is explained in the `description`.
// /// An Integer `error_code` field is also returned, but its contents are subject to change in the future.
// /// Some errors may also have an optional field `parameters` of the type `ResponseParameters`, which can help to automatically handle the error.
// ///
// /// See <https://core.telegram.org/bots/api#making-requests>
// pub struct ErrorResponse {
//     /// Always false
//     pub ok: bool,
//     pub description: String,
//     /// Contents are subject to change in the future
//     pub error_code: u64,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub parameters: Option<ResponseParameters>,
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// #[serde(untagged)]
// pub enum EditMessageResponse {
//     Message(MethodResponse<Message>),
//     Bool(MethodResponse<bool>),
// }