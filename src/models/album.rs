use crate::schema::*;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use std::convert::From;

use super::{artist::RichArtist, fetcher::FetcherAlbum, music::RichMusic};


#[derive(Debug, Serialize, Deserialize, QueryableByName, Identifiable, Queryable, Selectable, ToSchema)]
#[diesel(table_name = albums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Album {
    pub id: i32,
    pub title: String,
    pub artists_ids: Vec<i32>,
    pub description: Option<String>,
    pub youtube_id: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Insertable, QueryableByName, Queryable, Selectable, ToSchema)]
#[diesel(table_name = albums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAlbum {
    pub title: String,
    pub artists_ids: Vec<i32>,
    pub description: Option<String>
}

impl From<FetcherAlbum> for NewAlbum {
    fn from(fetcher_album: FetcherAlbum) -> Self {
        NewAlbum {
            title: fetcher_album.title.clone(),
            artists_ids: vec![1],
            description: Some("".to_owned())
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RichAlbum {
    pub id: i32,
    pub title: String,
    pub artists: Vec<RichArtist>,
    pub musics: Vec<RichMusic>,
    pub is_favorited: bool
}
