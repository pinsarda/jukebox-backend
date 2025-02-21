use crate::schema::*;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::models::album::Album;
use utoipa::ToSchema;


#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, ToSchema)]
#[diesel(table_name = artists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Insertable, QueryableByName, Queryable, Selectable, ToSchema)]
#[diesel(table_name = artists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewArtist {
    pub name: String,
    pub description: Option<String>
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RichArtist {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub is_favorited: bool
}
