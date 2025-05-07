use crate::{api::music, db_handlers::{album::add_album, artist::add_artist, music::add_music, user::create_user}, models::{album::NewAlbum, artist::NewArtist, music::NewMusic, user::NewUser}, DbBackend, DbConnection};
use diesel::prelude::*;
use std::{error::Error, result};

mod fetcher;
mod db_handlers;

fn load_env() {
    dotenvy::dotenv().ok();
}

pub fn establish_connection() -> DbConnection {
    load_env();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    DbConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn setup_test_db() -> Result<(), Box<dyn Error>> {
    let mut conn = establish_connection();
    
    let new_artist = NewArtist {
        name: "Test Artist".to_string(),
        description: None,
        youtube_id: None,
        spotify_id: None,
        deezer_id: None,
        apple_music_id: None
    };

    let first_artist = add_artist(&mut conn, new_artist.clone())?;
    let second_artist = add_artist(&mut conn, new_artist.clone())?;

    let new_album = NewAlbum {
        title: "Test Album".to_string(),
        artists_ids: vec![first_artist.id.clone(), second_artist.id.clone()],
        origin_user_id: 1,
        description: Some("Test description".to_string()),
        fetcher: None,
        youtube_id: None,
        spotify_id: None,
        deezer_id: None,
        apple_music_id: None
    };

    let album = add_album(&mut conn, new_album)?;

    let new_music = NewMusic {
        title: "Test Music".to_string(),
        artists_ids: vec![first_artist.id.clone(), second_artist.id.clone()],
        album_id: album.id.clone(),
        duration: 0,
        fetcher: None,
        youtube_id: None,
        spotify_id: None,
        deezer_id: None,
        apple_music_id: None
    };

    let _music = add_music(&mut conn, new_music)?;

    let new_user = NewUser {
        username: "Test User".to_string(),
        password: "password".to_string()
    };

    // If the user already exists, the test fails silently
    let _user = create_user(&mut conn, new_user);

    Ok(())
}
