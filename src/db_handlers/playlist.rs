use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::result::Error;
use crate::db_handlers::music::get_music_by_id;
use crate::db_handlers::music::get_playlist_musics;
use crate::models::playlist::InsertablePlaylist;
use crate::models::playlist::{ Playlist, RichPlaylist };
use crate::DbConnection;
use crate::db_handlers::user::get_user_by_id;



pub fn get_playlist_by_id(conn: &mut DbConnection, playlist_id: i32) -> Result<Playlist, Error> {
    use crate::schema::playlists::dsl::playlists;

    let playlist: Playlist = playlists
        .find(playlist_id)
        .select(Playlist::as_select())
        .first(conn)
        .expect("Error getting playlist_by_id");

    Ok(playlist)
}

pub fn to_rich_playlist(conn: &mut DbConnection, playlist: Playlist, user_id: i32) -> Result<RichPlaylist, Error> {

    let user = get_user_by_id(conn, user_id).expect("Error while getting user data");

    let musics = get_playlist_musics(conn, &playlist, user_id).expect("Error while getting playlist musics");

    let playlist_result = RichPlaylist {
        id: playlist.id,
        owner_id: playlist.owner_id,
        title: playlist.title,
        description: playlist.description,
        musics: musics,
        fetcher: playlist.fetcher,
        fetcher_id: playlist.fetcher_id,
        date_created: playlist.date_created
    };

    Ok(playlist_result)
}

pub fn create_playlist(conn: &mut DbConnection, new_playlist: InsertablePlaylist) -> Result<Playlist, Error> {
    use crate::schema::playlists::dsl::*;
    
    let inserted_playlist = diesel::insert_into(playlists)
        .values(&new_playlist)
        .get_result::<Playlist>(conn).unwrap();

    return Ok(inserted_playlist);
}

pub async fn search_playlists(conn: &mut DbConnection, query: &str, user_id: i32) -> Result<Vec<RichPlaylist>, Error> {
    use crate::schema::playlists::dsl::*;
    
    // Temporary solution until proper fuzzy searching is implemented for postgres
    let mut pattern_query = "%".to_string();
    pattern_query.push_str(query);
    pattern_query.push_str("%");

    let search_result: Vec<Playlist> = 
        playlists
        .filter(title.ilike(pattern_query))
        .limit(5)
        .select(Playlist::as_select())
        .load(conn)
        .expect("Error searching playlist");

    let results: Vec<RichPlaylist> = search_result.into_iter().map(|playlist| {
        to_rich_playlist(conn, playlist, user_id).unwrap()
    }).collect();

    Ok(results)
}

pub fn add_music_to_playlist(conn: &mut DbConnection, playlist_id: i32, music_id: i32) -> Result<(), Error> {
    use crate::schema::playlists::dsl::*;

    // Check if the music exists
    let music = get_music_by_id(conn, music_id)?;

    let mut playlist = get_playlist_by_id(conn, playlist_id)?;
    playlist.musics.push(music_id);
    
    diesel::update(playlists.find(playlist_id))
        .set(crate::schema::playlists::musics.eq(playlist.musics))
        .execute(conn)?;

    Ok(())
}