use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::result::Error;
use crate::db_handlers::artist::get_artists_by_ids;
use crate::models::album::Album;
use crate::models::music::{ Music, NewMusic, RichMusic };
use crate::DbConnection;
use crate::db_handlers::user::get_user_by_id;


pub fn get_music_by_id(conn: &mut DbConnection, music_id: i32) -> Result<Music, Error> {
    use crate::schema::musics::dsl::musics;

    let result = musics
        .find(music_id)    
        .select(Music::as_select())
        .first::<Music>(conn).expect("Error retrieving music from database");

    Ok(result)
}

pub fn to_rich_music(conn: &mut DbConnection, music: Music, user_id: i32) -> Result<RichMusic, Error> {
    use crate::schema::albums::dsl::albums;

    let album = albums
        .find(music.album_id)
        .select(Album::as_select())
        .first::<Album>(conn).expect("Error retrieving music's album from database");

    let user = get_user_by_id(conn, user_id).expect("Error while getting user data");

    let artists = get_artists_by_ids(conn, music.artists_ids, user_id).expect("Error while getting music artists");

    let music_result = RichMusic {
        id: music.id,
        title: music.title,
        artists: artists,
        album_id: music.album_id,
        album_title: album.title,
        is_favorited: user.favorite_musics.contains(&music.id)
    };
    
    Ok(music_result)
}

pub fn get_album_musics(conn: &mut DbConnection, album: &Album, user_id: i32) -> Result<Vec<RichMusic>, Error> {
    let album_musics: Vec<Music> = Music::belonging_to(album).select(Music::as_select()).load(conn)?;

    let results: Vec<RichMusic> = album_musics.into_iter().map(|music| {
        to_rich_music(conn, music, user_id).unwrap()
    }).collect();

    Ok(results)
}

pub fn add_music(conn: &mut DbConnection, new_music: NewMusic) -> Result<NewMusic, Error> {
    use crate::schema::musics::dsl::*;
    
    diesel::insert_into(musics)
        .values(&new_music)
        .execute(conn)
        .expect("Database error when inserting user");
    return Ok(new_music);
}

pub async fn search_musics(conn: &mut DbConnection, query: &str, user_id: i32) -> Result<Vec<RichMusic>, Error> {
    use crate::schema::musics::dsl::*;
    
    // Temporary solution until proper fuzzy searching is implemented for postgres
    let mut pattern_query = "%".to_string();
    pattern_query.push_str(query);
    pattern_query.push_str("%");

    let search_result: Vec<Music> = 
        musics
        .filter(title.ilike(pattern_query))
        .limit(5)
        .select(Music::as_select())
        .load(conn)
        .expect("Error searching music");

    let results: Vec<RichMusic> = search_result.into_iter().map(|music| {
            to_rich_music(conn, music, user_id).unwrap()
    }).collect();

    Ok(results)
}