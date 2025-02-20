use crate::schema::*;
use diesel::prelude::*;
use paperclip::actix::Apiv2Schema;
use serde::{Serialize, Deserialize};
use crate::models::album::Album;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Apiv2Schema)]
#[diesel(table_name = artists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Insertable, QueryableByName, Queryable, Selectable, Apiv2Schema)]
#[diesel(table_name = artists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewArtist {
    pub name: String,
    pub description: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct ArtistResult {
    pub id: i32,
    pub name: String,
    pub albums: Vec<Album>,
    pub is_favorited: bool
}
