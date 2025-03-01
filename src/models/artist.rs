use crate::schema::*;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use std::convert::From;

use crate::models::fetcher::FetcherArtist;


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

impl From<FetcherArtist> for NewArtist {
    fn from(fetcher_artist: FetcherArtist) -> Self {
        NewArtist {
            name: fetcher_artist.name.clone(),
            description: Some("".to_owned())
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RichArtist {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub is_favorited: bool
}
