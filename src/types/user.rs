// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::{Deserialize, Serialize};

/// This object represents a Telegram user or bot.
/// <https://core.telegram.org/bots/api#user>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    /// Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// True, if this user is a bot
    pub is_bot: bool,
    /// User's or bot's first name
    pub first_name: String,
    /// Optional. User's or bot's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. User's or bot's username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Optional. IETF language tag of the user's language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Optional. True, if the bot can be invited to groups. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<bool>,
    /// Optional. True, if privacy mode is disabled for the bot. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<bool>,
    /// Optional. True, if the bot supports inline queries. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<bool>,
}
