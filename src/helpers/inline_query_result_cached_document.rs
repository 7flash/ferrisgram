// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InlineQueryResultCachedDocument;

impl InlineQueryResultCachedDocument {
    /// This function creates an empty struct for the object InlineQueryResultCachedDocument.
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            title: "".to_string(),
            document_file_id: "".to_string(),
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}
impl Default for InlineQueryResultCachedDocument {
    fn default() -> Self {
        Self::new()
    }
}
