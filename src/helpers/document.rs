// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::Document;

impl Document {
    /// This function creates an empty struct for the object Document.
    pub fn new() -> Self {
        Self {
            file_id: "".to_string(),
            file_unique_id: "".to_string(),
            thumb: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        }
    }
}
impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}
