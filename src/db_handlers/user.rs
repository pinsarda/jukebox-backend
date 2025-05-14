use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::result::Error;
use crate::db_handlers::music::to_rich_music;
use crate::models::music::Music;
use crate::models::music::RichMusic;
use crate::models::artist::RichArtist;
use crate::models::album::RichAlbum;
use crate::models::user::{ User, NewUser, UserData };
use crate::DbConnection;
use argon2::Config;
use rand::Rng;
use crate::models::user::Favorites;

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

/// TODO : Adds an album to a user's list of favorites
pub fn add_favorite_album(conn: &mut DbConnection, user_id: i32, album_id: i32) -> Result<(), Error> {

    let user = get_user_by_id(conn, user_id)?;

    let mut new_favorites = user.favorite_albums.clone();
    if new_favorites.contains(&album_id) {
        return Ok(());
    }
    new_favorites.push(album_id);

    diesel::update(&user.clone())
        .set(crate::schema::users::favorite_albums.eq(new_favorites))
        .execute(conn)?;

    Ok(())
}

/// TODO : Remove an album from a user's list of favorites
pub fn remove_favorite_album(conn: &mut DbConnection, user_id: i32, album_id: i32) -> Result<(), Error> {

    let user = get_user_by_id(conn, user_id)?;

    let mut new_favorites = user.favorite_albums.clone();
    if !new_favorites.contains(&album_id) {
        return Ok(());
    }

    if let Some(pos) = new_favorites.iter().position(|x| *x == album_id) {
        new_favorites.remove(pos);
    }

    diesel::update(&user.clone())
        .set(crate::schema::users::favorite_albums.eq(new_favorites))
        .execute(conn)?;

    Ok(())
}

/// TODO : Adds an artist to a user's list of favorites
pub fn add_favorite_artist(conn: &mut DbConnection, user_id: i32, artist_id: i32) -> Result<(), Error> {

    let user = get_user_by_id(conn, user_id)?;

    let mut new_favorites = user.favorite_artists.clone();
    if new_favorites.contains(&artist_id) {
        return Ok(());
    }
    new_favorites.push(artist_id);

    diesel::update(&user.clone())
        .set(crate::schema::users::favorite_artists.eq(new_favorites))
        .execute(conn)?;

    Ok(())
}

/// TODO : Remove an artist from a user's list of favorites
pub fn remove_favorite_artist(conn: &mut DbConnection, user_id: i32, artist_id: i32) -> Result<(), Error> {

    let user = get_user_by_id(conn, user_id)?;

    let mut new_favorites = user.favorite_albums.clone();
    if !new_favorites.contains(&artist_id) {
        return Ok(());
    }

    if let Some(pos) = new_favorites.iter().position(|x| *x == artist_id) {
        new_favorites.remove(pos);
    }

    diesel::update(&user.clone())
        .set(crate::schema::users::favorite_artists.eq(new_favorites))
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

pub fn get_favorites(conn: &mut DbConnection, user_id: i32) -> Result<Favorites, Error> {
    use crate::schema::users::dsl::users;
    use crate::schema::musics::dsl::musics;
    use crate::schema::albums::dsl::albums;
    use crate::schema::artists::dsl::artists;
    use crate::db_handlers::album::to_rich_album;
    use crate::db_handlers::artist::to_rich_artist;
    use crate::models::album::Album;
    use crate::models::artist::Artist;
    use crate::models::user::Favorites;

    // Get the user to extract favorite IDs
    let user = get_user_by_id(conn, user_id)?;

    // Fetch favorite musics (same as original implementation)
    let favorite_musics: Vec<Music> = musics
        .filter(crate::schema::musics::id.eq_any(user.favorite_musics.clone()))
        .select(Music::as_select())
        .load::<Music>(conn)?;

    let rich_musics: Vec<RichMusic> = favorite_musics.into_iter().map(|music| {
        to_rich_music(conn, music, user_id).unwrap()
    }).collect();

    // Fetch favorite albums
    let favorite_albums: Vec<Album> = albums
        .filter(crate::schema::albums::id.eq_any(user.favorite_albums.clone()))
        .select(Album::as_select())
        .load::<Album>(conn)?;

    let rich_albums: Vec<RichAlbum> = favorite_albums.into_iter().map(|album| {
        to_rich_album(conn, album, user_id).unwrap()
    }).collect();

    // Fetch favorite artists
    let favorite_artists: Vec<Artist> = artists
        .filter(crate::schema::artists::id.eq_any(user.favorite_artists.clone()))
        .select(Artist::as_select())
        .load::<Artist>(conn)?;

    let rich_artists: Vec<RichArtist> = favorite_artists.into_iter().map(|artist| {
        to_rich_artist(conn, artist, user_id).unwrap()
    }).collect();

    // Combine into Favorites struct
    Ok(Favorites {
        musics: rich_musics,
        albums: rich_albums,
        artists: rich_artists
    })
}