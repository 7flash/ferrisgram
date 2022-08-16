// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::ChatAdministratorRights;

impl ChatAdministratorRights {
    /// This function creates an empty struct for the object ChatAdministratorRights.
    pub fn new() -> Self {
        Self {
            is_anonymous: false,
            can_manage_chat: false,
            can_delete_messages: false,
            can_manage_video_chats: false,
            can_restrict_members: false,
            can_promote_members: false,
            can_change_info: false,
            can_invite_users: false,
            can_post_messages: None,
            can_edit_messages: None,
            can_pin_messages: None,
        }
    }
}
impl Default for ChatAdministratorRights {
    fn default() -> Self {
        Self::new()
    }
}