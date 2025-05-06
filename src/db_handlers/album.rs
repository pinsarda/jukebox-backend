use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::result::Error;
use crate::db_handlers::music::get_album_musics;
use crate::models::album::{ Album, NewAlbum, RichAlbum };
use crate::DbConnection;
use crate::db_handlers::user::get_user_by_id;
use crate::db_handlers::artist::get_artists_by_ids;


pub fn get_album_by_id(conn: &mut DbConnection, album_id: i32) -> Result<Album, Error> {
    use crate::schema::albums::dsl::albums;

    let album: Album = albums
        .find(album_id)
        .select(Album::as_select())
        .first(conn)
        .expect("Error getting album_by_id");

    Ok(album)
}

pub fn get_album_by_title(conn: &mut DbConnection, album_title: String) -> Result<Album, Error> {
    use crate::schema::albums::*;

    let album_result = dsl::albums
        .filter(title.eq(album_title))
        .select(Album::as_select())
        .first(conn);

    album_result
}

pub fn to_rich_album(conn: &mut DbConnection, album: Album, user_id: i32) -> Result<RichAlbum, Error> {

    let user = get_user_by_id(conn, user_id).expect("Error while getting user data");

    let artists = get_artists_by_ids(conn, album.artists_ids.clone(), user_id).expect("Error while getting album artists");
    let musics = get_album_musics(conn, &album, user_id).expect("Error while getting album musics");

    let album_result = RichAlbum {
        id: album.id,
        title: album.title,
        artists: artists,
        musics: musics,
        fetcher: None,
        origin_user_id: album.origin_user_id,
        youtube_id: None,
        spotify_id: None,
        deezer_id: None,
        apple_music_id: None,
        is_favorited: user.favorite_albums.contains(&album.id)
    };

    Ok(album_result)
}

pub fn add_album(conn: &mut DbConnection, new_album: NewAlbum) -> Result<Album, Error> {
    use crate::schema::albums::dsl::*;
    
    let inserted_album = diesel::insert_into(albums)
        .values(&new_album)
        .get_result::<Album>(conn).unwrap();

    return Ok(inserted_album);
}

pub async fn search_albums(conn: &mut DbConnection, query: &str, user_id: i32) -> Result<Vec<RichAlbum>, Error> {
    use crate::schema::albums::dsl::*;
    
    // Temporary solution until proper fuzzy searching is implemented for postgres
    let mut pattern_query = "%".to_string();
    pattern_query.push_str(query);
    pattern_query.push_str("%");

    let search_result: Vec<Album> = 
        albums
        .filter(title.ilike(pattern_query))
        .limit(5)
        .select(Album::as_select())
        .load(conn)
        .expect("Error searching music");

    let results: Vec<RichAlbum> = search_result.into_iter().map(|music| {
        to_rich_album(conn, music, user_id).unwrap()
    }).collect();

    Ok(results)
}