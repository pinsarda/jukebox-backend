use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::result::Error;
use crate::models::user::{ User, NewUser, UserData };
use crate::DbConnection;
use argon2::Config;
use rand::Rng;

pub fn create_user(conn: &mut DbConnection, mut new_user: NewUser) -> Result<usize, Error> {
    use crate::schema::users::dsl::*;

    new_user.password = hash_password(new_user.password.clone()).unwrap();
    
    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
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
    use crate::schema::users::dsl::users;

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

pub fn get_user_data(conn: &mut DbConnection, user_id: i32) -> Result<UserData, Error> {
    use crate::schema::users::dsl::users;

    let user = users
        .find(user_id)
        .select(UserData::as_select())
        .first(conn)
        .optional();

    match user {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(Error::NotFound),
        Err(_) => Err(Error::NotFound)
    }
    
}

pub fn add_favorite_music(conn: &mut DbConnection, user_id: i32, music_id: i32) -> Result<(), Error> {

    let user = get_user_by_id(conn, user_id)?;

    let mut new_favorites = user.favorite_musics.clone();
    if new_favorites.contains(&music_id) {
        return Ok(());
    }
    new_favorites.push(music_id);

    diesel::update(&user.clone())
        .set(crate::schema::users::favorite_musics.eq(new_favorites))
        .execute(conn)?;

    Ok(())
}

pub fn remove_favorite_music(conn: &mut DbConnection, user_id: i32, music_id: i32) -> Result<(), Error> {

    let user = get_user_by_id(conn, user_id)?;

    let mut new_favorites = user.favorite_musics.clone();
    if !new_favorites.contains(&music_id) {
        return Ok(());
    }

    if let Some(pos) = new_favorites.iter().position(|x| *x == music_id) {
        new_favorites.remove(pos);
    }

    diesel::update(&user.clone())
        .set(crate::schema::users::favorite_musics.eq(new_favorites))
        .execute(conn)?;

    Ok(())
}

pub fn hash_password(password: String) -> Result<String, argon2::Error> {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();
    
    argon2::hash_encoded(password.as_bytes(), &salt, &config)
}

pub fn verify_password(hash: String, password: String) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(&hash, password.as_bytes())
}