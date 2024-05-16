use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportElementErrorDataField {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorDataFieldType,
    pub field_name: String,
    pub data_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportElementErrorFrontSide {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorFrontSideType,
    pub file_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportElementErrorReverseSide {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorReverseSideType,
    pub file_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportElementErrorSelfie {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorSelfieType,
    pub file_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportElementErrorFile {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorFileType,
    pub file_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportElementErrorFiles {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorFileType,
    pub file_hashes: Vec<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportElementErrorTranslationFile {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorTranslationFileType,
    pub file_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportElementErrorTranslationFiles {
    #[serde(rename = "type")]
    pub type_field: PassportElementErrorTranslationFileType,
    pub file_hashes: Vec<String>,
    pub message: String,
}
