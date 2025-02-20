use crate::schema::*;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::models::music::MusicResult;
use utoipa::ToSchema;


#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Selectable, ToSchema)]
#[diesel(table_name = albums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Album {
    pub id: i32,
    pub title: String,
    pub artists_ids: Vec<i32>,
    pub description: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Insertable, QueryableByName, Queryable, Selectable, ToSchema)]
#[diesel(table_name = albums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAlbum {
    pub title: String,
    pub artists_ids: Vec<i32>,
    pub description: Option<String>
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AlbumResult {
    pub id: i32,
    pub title: String,
    pub artists_ids: Vec<i32>,
    pub musics: Vec<MusicResult>,
    pub is_favorited: bool
}
