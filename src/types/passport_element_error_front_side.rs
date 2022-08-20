// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::{Deserialize, Serialize};


/// Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
/// <https://core.telegram.org/bots/api#passportelementerrorfrontside>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PassportElementErrorFrontSide {
    /// The section of the user's Telegram Passport which has the issue, one of "passport", "driver_license", "identity_card", "internal_passport"
    pub r#type: String,
    /// Base64-encoded hash of the file with the front side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}