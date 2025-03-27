use crate::schema::*;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use crate::models::album::Album;

use super::artist::RichArtist;

#[derive(Debug, Serialize, Deserialize)]
pub struct YoutubeVideo {
    pub id: i32,
    pub url: String,
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Identifiable, Associations, QueryableByName, Queryable, Selectable, ToSchema)]
#[diesel(table_name = musics)]
#[diesel(belongs_to(Album))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Music {
    pub id: i32,
    pub title: String,
    pub artists_ids: Vec<i32>,
    pub album_id: i32,
    pub duration: i32,
    pub youtube_id: Option<String>,
    pub spotify_id: Option<String>,
    pub deezer_id: Option<String>,
    pub apple_music_id: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Insertable, QueryableByName, Queryable, Selectable, ToSchema)]
#[diesel(table_name = musics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMusic {
    pub title: String,
    pub artists_ids: Vec<i32>,
    pub album_id: i32,
    pub duration: i32,
    pub youtube_id: Option<String>,
    pub spotify_id: Option<String>,
    pub deezer_id: Option<String>,
    pub apple_music_id: Option<String>
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RichMusic {
    pub id: i32,
    pub title: String,
    pub artists: Vec<RichArtist>,
    pub album_id: i32,
    pub duration: i32,
    pub album_title: String,
    pub youtube_id: Option<String>,
    pub spotify_id: Option<String>,
    pub deezer_id: Option<String>,
    pub apple_music_id: Option<String>,
    pub is_favorited: bool
}
