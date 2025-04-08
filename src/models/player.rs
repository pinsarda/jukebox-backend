use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::music::RichMusic;
use crate::models::music::Music;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct PlayerState {
    pub queue: Vec<Music>,
    pub queue_index: i32,
    pub current_pos: i32,
    pub volume: f32,
    pub is_playing: bool
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct RichPlayerState {
    pub queue: Vec<RichMusic>,
    pub queue_index: i32,
    pub current_pos: i32,
    pub volume: f32,
    pub is_playing: bool
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct SeekRequest {
    pub pos: u64
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct VolumeChangeRequest {
    pub volume: f32
}
