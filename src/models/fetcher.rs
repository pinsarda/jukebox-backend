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
pub enum FetcherSearchResult {
    Music(FetcherMusic),
    Album(FetcherAlbum),
    Artist(FetcherArtist),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ExternalIds {
    pub youtube_id: Option<String>,
    pub spotify_id: Option<String>,
    pub deezer_id: Option<String>,
    pub apple_music_id: Option<String>
}