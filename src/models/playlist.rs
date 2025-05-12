use std::time::SystemTime;

use crate::schema::*;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

use super::music::RichMusic;

#[derive(Debug, Serialize, Deserialize, Clone, Identifiable, Queryable, Selectable)]
#[diesel(table_name = playlists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Playlist {
    pub id: i32,
    pub owner_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub musics: Vec<i32>,
    pub fetcher: Option<String>,
    pub fetcher_id: Option<String>,
    pub date_created: SystemTime
}

#[derive(Debug, Serialize, Deserialize, Insertable, QueryableByName, Queryable, Selectable, ToSchema)]
#[diesel(table_name = playlists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPlaylist {
    pub title: String,
    pub description: Option<String>,
    pub fetcher: Option<String>,
    pub fetcher_id: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Insertable, QueryableByName, Queryable, Selectable)]
#[diesel(table_name = playlists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertablePlaylist {
    pub owner_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub fetcher: Option<String>,
    pub fetcher_id: Option<String>,
    pub date_created: SystemTime
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RichPlaylist {
    pub id: i32,
    pub owner_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub musics: Vec<RichMusic>,
    pub fetcher: Option<String>,
    pub fetcher_id: Option<String>,
    pub date_created: SystemTime
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct MusicAddRequest {
    pub playlist_id: i32,
    pub music_id: i32
}