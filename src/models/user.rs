use crate::schema::*;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

use super::{album::RichAlbum, artist::RichArtist, music::RichMusic};

#[derive(Debug, Serialize, Deserialize, Queryable, AsChangeset, Identifiable,  Clone, Selectable, ToSchema)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub favorite_musics: Vec<i32>,
    pub favorite_albums: Vec<i32>,
    pub favorite_artists: Vec<i32>,
    pub playlists_library: Vec<i32>
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, ToSchema)]
#[diesel(table_name = users)]
pub struct UserData {
    pub username: String
}

#[derive(Debug, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Favorites {
    pub artists: Vec<RichArtist>,
    pub albums: Vec<RichAlbum>,
    pub musics: Vec<RichMusic>
}
