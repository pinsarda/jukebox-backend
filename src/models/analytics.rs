use crate::schema::*;
use std::time::SystemTime;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, QueryableByName, Identifiable, Queryable, Selectable)]
#[diesel(table_name = analytics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Analytics {
    pub id: i32,
    pub music_id: i32,
    pub album_id: i32,
    pub user_id: i32,
    pub date_played: SystemTime
}

#[derive(Debug, Serialize, Deserialize, QueryableByName, Insertable, Queryable, Selectable)]
#[diesel(table_name = analytics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAnalytics {
    pub music_id: i32,
    pub album_id: i32,
    pub user_id: i32,
    pub date_played: SystemTime
}