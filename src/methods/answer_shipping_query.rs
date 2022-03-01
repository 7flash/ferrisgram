// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::ShippingOption;
use crate::Bot;

impl Bot {
    /// If you sent an invoice requesting a shipping address and the parameter is_flexible was specified, the Bot API will send an Update with a shipping_query field to the bot. Use this method to reply to shipping queries. On success, True is returned.
    /// <https://core.telegram.org/bots/api#answershippingquery>
    pub fn answer_shipping_query(
        &self,
        shipping_query_id: String,
        ok: bool,
    ) -> AnswerShippingQueryBuilder {
        AnswerShippingQueryBuilder::new(self, shipping_query_id, ok)
    }
}

#[derive(Serialize)]
pub struct AnswerShippingQueryBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the query to be answered
    pub shipping_query_id: String,
    /// Specify True if delivery to the specified address is possible and False if there are any problems (for example, if delivery to the specified address is not possible)
    pub ok: bool,
    /// Required if ok is True. A JSON-serialized array of available shipping options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<ShippingOption>>,
    /// Required if ok is False. Error message in human readable form that explains why it is impossible to complete the order (e.g. "Sorry, delivery to your desired address is unavailable'). Telegram will display this message to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl<'a> AnswerShippingQueryBuilder<'a> {
    pub fn new(bot: &'a Bot, shipping_query_id: String, ok: bool) -> Self {
        Self {
            bot,
            shipping_query_id,
            ok,
            shipping_options: None,
            error_message: None,
        }
    }

    pub fn shipping_query_id(mut self, shipping_query_id: String) -> Self {
        self.shipping_query_id = shipping_query_id;
        self
    }

    pub fn ok(mut self, ok: bool) -> Self {
        self.ok = ok;
        self
    }

    pub fn shipping_options(mut self, shipping_options: Vec<ShippingOption>) -> Self {
        self.shipping_options = Some(shipping_options);
        self
    }

    pub fn error_message(mut self, error_message: String) -> Self {
        self.error_message = Some(error_message);
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot
            .get::<bool>("answerShippingQuery", Some(&form))
            .await
    }
}
