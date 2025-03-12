use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{album::RichAlbum, artist::RichArtist, music::RichMusic};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SearchQuery {
    pub query: String
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct SearchResult {
    pub musics: Vec<RichMusic>,
    pub albums: Vec<RichAlbum>,
    pub artists: Vec<RichArtist>
}