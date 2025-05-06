use std::time::SystemTime;

use crate::{models::analytics::{Analytics, NewAnalytics}, DbConnection};
use diesel::result::Error;
use diesel::RunQueryDsl;
use diesel::prelude::*;

pub fn log_playback(conn: &mut DbConnection, music_id: i32, album_id: i32, user_id: i32) -> Result<Analytics, Error> {
    use crate::schema::analytics::dsl::analytics;

    let new_analytic = NewAnalytics {
        music_id,
        album_id,
        user_id,
        date_played: SystemTime::now()
    };
    
    let analytic = diesel::insert_into(analytics)
        .values(&new_analytic)
        .get_result::<Analytics>(conn)?;

    return Ok(analytic);
}