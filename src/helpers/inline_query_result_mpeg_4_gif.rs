// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InlineQueryResultMpeg4Gif;

impl InlineQueryResultMpeg4Gif {
    /// This function creates an empty struct for the object InlineQueryResultMpeg4Gif.
    pub fn new() -> Self {
        Self {
            r#type: "".to_string(),
            id: "".to_string(),
            mpeg4_url: "".to_string(),
            mpeg4_width: None,
            mpeg4_height: None,
            mpeg4_duration: None,
            thumb_url: "".to_string(),
            thumb_mime_type: None,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}
impl Default for InlineQueryResultMpeg4Gif {
    fn default() -> Self {
        Self::new()
    }
}