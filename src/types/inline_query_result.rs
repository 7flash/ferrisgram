// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::{InlineQueryResultCachedAudio, InlineQueryResultCachedDocument, InlineQueryResultCachedGif, InlineQueryResultCachedMpeg4Gif, InlineQueryResultCachedPhoto, InlineQueryResultCachedSticker, InlineQueryResultCachedVideo, InlineQueryResultCachedVoice, InlineQueryResultArticle, InlineQueryResultAudio, InlineQueryResultContact, InlineQueryResultGame, InlineQueryResultDocument, InlineQueryResultGif, InlineQueryResultLocation, InlineQueryResultMpeg4Gif, InlineQueryResultPhoto, InlineQueryResultVenue, InlineQueryResultVideo, InlineQueryResultVoice};
use serde::{Deserialize, Serialize};


/// This object represents one result of an inline query. Telegram clients currently support results of the following 20 types:
/// - InlineQueryResultCachedAudio
/// - InlineQueryResultCachedDocument
/// - InlineQueryResultCachedGif
/// - InlineQueryResultCachedMpeg4Gif
/// - InlineQueryResultCachedPhoto
/// - InlineQueryResultCachedSticker
/// - InlineQueryResultCachedVideo
/// - InlineQueryResultCachedVoice
/// - InlineQueryResultArticle
/// - InlineQueryResultAudio
/// - InlineQueryResultContact
/// - InlineQueryResultGame
/// - InlineQueryResultDocument
/// - InlineQueryResultGif
/// - InlineQueryResultLocation
/// - InlineQueryResultMpeg4Gif
/// - InlineQueryResultPhoto
/// - InlineQueryResultVenue
/// - InlineQueryResultVideo
/// - InlineQueryResultVoice
/// Note: All URLs passed in inline query results will be available to end users and therefore must be assumed to be public.
/// <https://core.telegram.org/bots/api#inlinequeryresult>
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum InlineQueryResult {
    InlineQueryResultCachedAudio(InlineQueryResultCachedAudio),
    InlineQueryResultCachedDocument(InlineQueryResultCachedDocument),
    InlineQueryResultCachedGif(InlineQueryResultCachedGif),
    InlineQueryResultCachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    InlineQueryResultCachedPhoto(InlineQueryResultCachedPhoto),
    InlineQueryResultCachedSticker(InlineQueryResultCachedSticker),
    InlineQueryResultCachedVideo(InlineQueryResultCachedVideo),
    InlineQueryResultCachedVoice(InlineQueryResultCachedVoice),
    #[serde(rename = "article")]
    InlineQueryResultArticle(InlineQueryResultArticle),
    #[serde(rename = "audio")]
    InlineQueryResultAudio(InlineQueryResultAudio),
    #[serde(rename = "contact")]
    InlineQueryResultContact(InlineQueryResultContact),
    #[serde(rename = "game")]
    InlineQueryResultGame(InlineQueryResultGame),
    #[serde(rename = "document")]
    InlineQueryResultDocument(InlineQueryResultDocument),
    #[serde(rename = "gif")]
    InlineQueryResultGif(InlineQueryResultGif),
    #[serde(rename = "location")]
    InlineQueryResultLocation(InlineQueryResultLocation),
    #[serde(rename = "mpeg4_gif")]
    InlineQueryResultMpeg4Gif(InlineQueryResultMpeg4Gif),
    #[serde(rename = "photo")]
    InlineQueryResultPhoto(InlineQueryResultPhoto),
    #[serde(rename = "venue")]
    InlineQueryResultVenue(InlineQueryResultVenue),
    #[serde(rename = "video")]
    InlineQueryResultVideo(InlineQueryResultVideo),
    #[serde(rename = "voice")]
    InlineQueryResultVoice(InlineQueryResultVoice),
}
