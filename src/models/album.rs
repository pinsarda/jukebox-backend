use crate::schema::*;
use diesel::prelude::*;
use paperclip::actix::Apiv2Schema;
use serde::{Serialize, Deserialize};
use crate::models::music::MusicResult;


#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Selectable, Apiv2Schema)]
#[diesel(table_name = albums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Album {
    pub id: i32,
    pub title: String,
    pub artists_ids: Vec<i32>,
    pub description: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Insertable, QueryableByName, Queryable, Selectable, Apiv2Schema)]
#[diesel(table_name = albums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAlbum {
    pub title: String,
    pub artists_ids: Vec<i32>,
    pub description: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct AlbumResult {
    pub id: i32,
    pub title: String,
    pub artists_ids: Vec<i32>,
    pub musics: Vec<MusicResult>,
    pub is_favorited: bool
}
