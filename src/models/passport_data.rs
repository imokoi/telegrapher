use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportFile {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: u64,
    pub file_date: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    pub type_field: String,
    pub data: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub files: Option<Vec<PassportFile>>,
    pub front_side: Option<PassportFile>,
    pub reverse_side: Option<PassportFile>,
    pub selfie: Option<PassportFile>,
    pub translation: Option<Vec<PassportFile>>,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportElementErrorUnspecified {
    #[serde(rename = "type")]
    pub type_field: String,
    pub element_hash: String,
    pub message: String,
}
