// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::EncryptedCredentials;
use crate::types::PassportData;

impl PassportData {
    /// This function creates an empty struct for the object PassportData.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            credentials: EncryptedCredentials::new(),
        }
    }
}
impl Default for PassportData {
    fn default() -> Self {
        Self::new()
    }
}
