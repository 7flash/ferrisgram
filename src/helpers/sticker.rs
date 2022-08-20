// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::Sticker;

impl Sticker {
    /// This function creates an empty struct for the object Sticker.
    pub fn new() -> Self {
        Self {
            file_id: "".to_string(),
            file_unique_id: "".to_string(),
            r#type: "".to_string(),
            width: 0,
            height: 0,
            is_animated: false,
            is_video: false,
            thumb: None,
            emoji: None,
            set_name: None,
            premium_animation: None,
            mask_position: None,
            custom_emoji_id: None,
            file_size: None,
        }
    }
}
impl Default for Sticker {
    fn default() -> Self {
        Self::new()
    }
}