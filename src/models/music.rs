use crate::schema::*;
use diesel::prelude::*;
use paperclip::{actix::Apiv2Schema, v2::serde_json};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct YoutubeVideo {
    pub id: i32,
    pub url: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Apiv2Schema)]
#[diesel(table_name = albums)]
pub struct Album {
    pub id: i32,
    pub title: String,
    pub artists_ids: String,
    description: String
}

#[derive(Debug, Serialize, Deserialize, QueryableByName, Queryable, Selectable, Apiv2Schema)]
#[diesel(table_name = musics)]
pub struct Music {
    pub id: i32,
    pub title: String,
    pub artists_ids: String,
    pub album_id: i32
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Apiv2Schema)]
#[diesel(table_name = artists)]
pub struct Artist {
    pub id: i32,
    pub name: String,
    description: String
}