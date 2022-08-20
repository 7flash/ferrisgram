// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::Bot;
use crate::error::Result;
use crate::types::BotCommandScope;

impl Bot {
    /// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, higher level commands will be shown to affected users. Returns True on success.
    /// <https://core.telegram.org/bots/api#deletemycommands>
    pub fn delete_my_commands(&self) -> DeleteMyCommandsBuilder {
        DeleteMyCommandsBuilder::new(self)
    }
}

#[derive(Serialize)]
pub struct DeleteMyCommandsBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to BotCommandScopeDefault.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}


impl <'a> DeleteMyCommandsBuilder<'a> {
    pub fn new(bot: &'a Bot) -> Self {
        Self{
            bot,
            scope: None,
            language_code: None,
        }
    }

    pub fn scope(mut self, scope: BotCommandScope) -> Self {
        self.scope = Some(scope);
        self
    }
                
    pub fn language_code(mut self, language_code: String) -> Self {
        self.language_code = Some(language_code);
        self
    }
                
    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("deleteMyCommands", Some(&form)).await
    }

}