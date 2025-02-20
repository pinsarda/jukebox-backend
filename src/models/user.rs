use crate::schema::*;
use diesel::prelude::*;
use paperclip::actix::Apiv2Schema;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Apiv2Schema)]
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

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Apiv2Schema)]
#[diesel(table_name = users)]
pub struct UserData {
    pub username: String
}

#[derive(Debug, Serialize, Deserialize, Insertable, Apiv2Schema)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String
}
