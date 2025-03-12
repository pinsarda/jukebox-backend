use crate::schema::*;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use std::convert::From;

use crate::models::fetcher::FetcherArtist;


#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, ToSchema)]
#[diesel(table_name = artists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub youtube_id: Option<String>,
    pub spotify_id: Option<String>,
    pub deezer_id: Option<String>,
    pub apple_music_id: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Insertable, QueryableByName, Queryable, Selectable, ToSchema)]
#[diesel(table_name = artists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewArtist {
    pub name: String,
    pub description: Option<String>,
    pub youtube_id: Option<String>,
    pub spotify_id: Option<String>,
    pub deezer_id: Option<String>,
    pub apple_music_id: Option<String>
}

impl From<FetcherArtist> for NewArtist {
    fn from(fetcher_artist: FetcherArtist) -> Self {
        NewArtist {
            name: fetcher_artist.name.clone(),
            description: None,
            youtube_id: None,
            spotify_id: None,
            deezer_id: None,
            apple_music_id: None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct RichArtist {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub is_favorited: bool
}
