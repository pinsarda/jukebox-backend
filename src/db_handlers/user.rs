use diesel::sql_types::Integer;
use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::result::Error;
use crate::api::user;
use crate::models::user::{ User, NewUser };
use crate::DbConnection;

pub fn create_user(conn: &mut DbConnection, new_user: NewUser) -> Result<NewUser, Error> {
    use crate::schema::users::dsl::*;
    
    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .expect("Database error when inserting user");
    return Ok(new_user);
}

pub fn get_user(conn: &mut DbConnection, user_data: NewUser) -> Result<User, Error> {
    use crate::schema::users::dsl::{username, users};

    let mut items = users
        .filter(username.eq(&user_data.username))
        .load::<User>(conn)?;

    if let Some(user) = items.pop() {
        return Ok(user);
    }

    Err(Error::NotFound)
}

pub fn get_user_by_id(conn: &mut DbConnection, user_id: i32) -> Result<User, Error> {
    use crate::schema::users::dsl::{users};

    let user = users
        .find(user_id)
        .select(User::as_select())
        .first(conn)
        .optional();

    match user {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(Error::NotFound),
        Err(_) => Err(Error::NotFound)
    }
    
}