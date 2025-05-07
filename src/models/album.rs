use crate::schema::*;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use std::convert::From;

use super::{artist::RichArtist, music::RichMusic};


#[derive(Debug, Serialize, Deserialize, Clone, QueryableByName, Identifiable, Queryable, Selectable, ToSchema)]
#[diesel(table_name = albums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Album {
    pub id: i32,
    pub title: String,
    pub artists_ids: Vec<i32>,
    pub description: Option<String>,
    pub fetcher: Option<String>,
    pub origin_user_id: i32,
    pub youtube_id: Option<String>,
    pub spotify_id: Option<String>,
    pub deezer_id: Option<String>,
    pub apple_music_id: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Insertable, QueryableByName, Queryable, Selectable, ToSchema)]
#[diesel(table_name = albums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAlbum {
    pub title: String,
    pub artists_ids: Vec<i32>,
    pub description: Option<String>,
    pub fetcher: Option<String>,
    pub origin_user_id: i32,
    pub youtube_id: Option<String>,
    pub spotify_id: Option<String>,
    pub deezer_id: Option<String>,
    pub apple_music_id: Option<String>
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RichAlbum {
    pub id: i32,
    pub title: String,
    pub artists: Vec<RichArtist>,
    pub musics: Vec<RichMusic>,
    pub is_favorited: bool,
    pub fetcher: Option<String>,
    pub origin_user_id: i32,
    pub youtube_id: Option<String>,
    pub spotify_id: Option<String>,
    pub deezer_id: Option<String>,
    pub apple_music_id: Option<String>
}
