// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};
use serde::{Deserialize, Serialize};

/// Represents a link to a file. By default, this file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the file. Currently, only .PDF and .ZIP files can be sent using this method.
/// Note: This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
/// <https://core.telegram.org/bots/api#inlinequeryresultdocument>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InlineQueryResultDocument {
    /// Type of the result, must be document
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Title for the result
    pub title: String,
    /// Optional. Caption of the document to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the document caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// A valid URL for the file
    pub document_url: String,
    /// Mime type of the content of the file, either "application/pdf" or "application/zip"
    pub mime_type: String,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Optional. URL of the thumbnail (JPEG only) for the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    /// Optional. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    /// Optional. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}
