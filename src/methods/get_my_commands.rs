// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::BotCommand;
use crate::types::BotCommandScope;
use crate::Bot;

impl Bot {
    /// Use this method to get the current list of the bot's commands for the given scope and user language. Returns Array of BotCommand on success. If commands aren't set, an empty list is returned.
    /// <https://core.telegram.org/bots/api#getmycommands>
    pub fn get_my_commands(&self) -> GetMyCommandsBuilder {
        GetMyCommandsBuilder::new(self)
    }
}

#[derive(Serialize)]
pub struct GetMyCommandsBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// A JSON-serialized object, describing scope of users. Defaults to BotCommandScopeDefault.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    /// A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl<'a> GetMyCommandsBuilder<'a> {
    pub fn new(bot: &'a Bot) -> Self {
        Self {
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

    pub async fn send(self) -> Result<Vec<BotCommand>> {
        let form = serde_json::to_value(&self)?;
        self.bot
            .get::<Vec<BotCommand>>("getMyCommands", Some(&form))
            .await
    }
}
