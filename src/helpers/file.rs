// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::File;

impl File {
    /// This function creates an empty struct for the object File.
    pub fn new() -> Self {
        Self {
            file_id: "".to_string(),
            file_unique_id: "".to_string(),
            file_size: None,
            file_path: None,
        }
    }
}
impl Default for File {
    fn default() -> Self {
        Self::new()
    }
}
