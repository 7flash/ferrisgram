// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::Contact;

impl Contact {
    /// This function creates an empty struct for the object Contact.
    pub fn new() -> Self {
        Self {
            phone_number: "".to_string(),
            first_name: "".to_string(),
            last_name: None,
            user_id: None,
            vcard: None,
        }
    }
}
impl Default for Contact {
    fn default() -> Self {
        Self::new()
    }
}