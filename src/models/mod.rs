pub mod user;
pub mod music;
pub mod album;
pub mod artist;
pub mod fetcher;
pub mod errors;
pub mod search;
pub mod player;
pub mod analytics;
pub mod playlist;

use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Id {
    pub id: i32
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Index {
    pub index: i32
}