// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::{Deserialize, Serialize};


/// Represents the default scope of bot commands. Default commands are used if no commands with a narrower scope are specified for the user.
/// <https://core.telegram.org/bots/api#botcommandscopedefault>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotCommandScopeDefault {
    /// Scope type, must be default
    pub r#type: String,
}