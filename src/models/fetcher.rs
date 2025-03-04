use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Serialize, Deserialize, ToSchema, Debug)]
pub struct FetcherMusic {
    pub fetcher_id: Option<String>,
    pub title: String,
    pub artists: Vec<FetcherArtist>
}

#[derive(Clone, Serialize, Deserialize, ToSchema, Debug)]
pub struct FetcherAlbum {
    pub fetcher_id: Option<String>,
    pub title: String,
    pub musics: Vec<FetcherMusic>,
    pub artists: Vec<FetcherArtist>
}

#[derive(Clone, Serialize, Deserialize, ToSchema, Debug)]
pub struct FetcherArtist {
    pub fetcher_id: Option<String>,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, ToSchema, Debug)]
pub struct FetcherQueryData {
    pub fetcher_id: Option<String>,
    pub title: String,
    pub album_title: String,
    pub artist_name: String
}

#[derive(Clone, Serialize, Deserialize, ToSchema, Debug)]
pub enum SearchResult {
    Music(FetcherMusic),
    Album(FetcherAlbum),
    Artist(FetcherArtist),
}