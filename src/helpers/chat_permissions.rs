// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::ChatPermissions;

impl ChatPermissions {
    /// This function creates an empty struct for the object ChatPermissions.
    pub fn new() -> Self {
        Self {
            can_send_messages: None,
            can_send_media_messages: None,
            can_send_polls: None,
            can_send_other_messages: None,
            can_add_web_page_previews: None,
            can_change_info: None,
            can_invite_users: None,
            can_pin_messages: None,
        }
    }
}
impl Default for ChatPermissions {
    fn default() -> Self {
        Self::new()
    }
}