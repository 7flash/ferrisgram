// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InputContactMessageContent;

impl InputContactMessageContent {
    /// This function creates an empty struct for the object InputContactMessageContent.
    pub fn new() -> Self {
        Self {
            phone_number: "".to_string(),
            first_name: "".to_string(),
            last_name: None,
            vcard: None,
        }
    }
}
impl Default for InputContactMessageContent {
    fn default() -> Self {
        Self::new()
    }
}