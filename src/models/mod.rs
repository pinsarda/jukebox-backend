pub mod user;
pub mod music;
pub mod album;
pub mod artist;
pub mod fetcher;

use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Id {
    pub id: i32
}