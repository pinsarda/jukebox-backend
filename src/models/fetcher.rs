use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub struct FetcherMusic {
    pub fetcher_id: String,
    pub title: String,
    pub artists: Vec<FetcherArtist>
}

pub struct FetcherAlbum {
    pub fetcher_id: String,
    pub title: String,
    pub musics: Vec<FetcherMusic>,
    pub artists: Vec<FetcherArtist>
}

#[derive(Clone)]
pub struct FetcherArtist {
    pub fetcher_id: String,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, ToSchema)]
pub struct FetcherQueryData {
    pub fetcher_id: Option<String>,
    pub title: String,
    pub album_title: String,
    pub artist_name: String
}