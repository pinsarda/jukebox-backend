pub mod user;
pub mod music;
pub mod album;
pub mod artist;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    pub id: i32
}
