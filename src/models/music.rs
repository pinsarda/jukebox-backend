use crate::schema::*;
use diesel::{prelude::*, sql_types::Bool};
use paperclip::{actix::Apiv2Schema, v2::serde_json};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct YoutubeVideo {
    pub id: i32,
    pub url: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Identifiable, QueryableByName, Queryable, Selectable, Apiv2Schema)]
#[diesel(table_name = musics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Music {
    pub id: i32,
    pub title: String,
    pub artists_ids: Vec<i32>,
    pub album_id: i32
}

#[derive(Debug, Serialize, Deserialize, Insertable, QueryableByName, Queryable, Selectable, Apiv2Schema)]
#[diesel(table_name = musics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMusic {
    pub title: String,
    pub artists_ids: Vec<i32>,
    pub album_id: i32
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct MusicResult {
    pub id: i32,
    pub title: String,
    pub artists_ids: Vec<i32>,
    pub album_id: i32,
    pub album_title: String,
    pub is_favorited: bool
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Apiv2Schema)]
#[diesel(table_name = artists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}