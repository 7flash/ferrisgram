// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::{
    CallbackQuery, ChatJoinRequest, ChatMemberUpdated, ChosenInlineResult, InlineQuery, Message,
    Poll, PollAnswer, PreCheckoutQuery, ShippingQuery,
};
use serde::{Deserialize, Serialize};

/// This object represents an incoming update.At most one of the optional parameters can be present in any given update.
/// <https://core.telegram.org/bots/api#update>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Update {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you're using Webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// Optional. New incoming message of any kind - text, photo, sticker, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<Message>>,
    /// Optional. New version of a message that is known to the bot and was edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_message: Option<Box<Message>>,
    /// Optional. New incoming channel post of any kind - text, photo, sticker, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_post: Option<Box<Message>>,
    /// Optional. New version of a channel post that is known to the bot and was edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_channel_post: Option<Box<Message>>,
    /// Optional. New incoming inline query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_query: Option<InlineQuery>,
    /// Optional. The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_inline_result: Option<ChosenInlineResult>,
    /// Optional. New incoming callback query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<CallbackQuery>,
    /// Optional. New incoming shipping query. Only for invoices with flexible price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_query: Option<ShippingQuery>,
    /// Optional. New incoming pre-checkout query. Contains full information about checkout
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    /// Optional. New poll state. Bots receive only updates about stopped polls and polls, which are sent by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    /// Optional. A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_answer: Option<PollAnswer>,
    /// Optional. The bot's chat member status was updated in a chat. For private chats, this update is received only when the bot is blocked or unblocked by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_chat_member: Option<ChatMemberUpdated>,
    /// Optional. A chat member's status was updated in a chat. The bot must be an administrator in the chat and must explicitly specify "chat_member" in the list of allowed_updates to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_member: Option<ChatMemberUpdated>,
    /// Optional. A request to join the chat has been sent. The bot must have the can_invite_users administrator right in the chat to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_join_request: Option<ChatJoinRequest>,
}
