// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::{Deserialize, Serialize};

/// Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
/// <https://core.telegram.org/bots/api#passportelementerrortranslationfiles>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PassportElementErrorTranslationFiles {
    /// Error source, must be translation_files
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue, one of "passport", "driver_license", "identity_card", "internal_passport", "utility_bill", "bank_statement", "rental_agreement", "passport_registration", "temporary_registration"
    pub r#type: String,
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}
