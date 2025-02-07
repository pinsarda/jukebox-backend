use crate::schema::*;
use diesel::prelude::*;
use paperclip::actix::Apiv2Schema;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Apiv2Schema)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub userdata: String
}

#[derive(Debug, Serialize, Deserialize, Insertable, Apiv2Schema)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub userdata: String
}
