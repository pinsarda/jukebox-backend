use diesel::RunQueryDsl;
use diesel::result::Error;
use crate::models::user::{ User, NewUser};
use crate::DbConnection;

pub fn create_user(conn: &mut DbConnection, new_user: NewUser) -> Result<NewUser, Error> {
    use crate::schema::users::dsl::*;
    
    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .expect("Database error when inserting user");
    return Ok(new_user);
}