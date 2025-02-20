use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::result::Error;
use crate::db_handlers::music::get_album_musics;
use crate::models::album::{ Album, NewAlbum, RichAlbum };
use crate::models::music::Music;
use crate::models::music::RichMusic;
use crate::DbConnection;
use crate::db_handlers::user::get_user_by_id;

pub fn get_album_by_id(conn: &mut DbConnection, album_id: i32) -> Result<Album, Error> {
    use crate::schema::albums::dsl::albums;

    let album: Album = albums
        .find(album_id)
        .select(Album::as_select())
        .first(conn)
        .expect("Error getting album_by_id");

    Ok(album)
    
}

pub fn to_rich_album(conn: &mut DbConnection, album: Album, user_id: i32) -> Result<RichAlbum, Error> {

    let user = get_user_by_id(conn, user_id).expect("Error while getting user data");

    let album_result = RichAlbum {
        id: album.id,
        title: album.title.clone(),
        artists_ids: album.artists_ids.clone(),
        musics: get_album_musics(conn, &album, user_id).expect(""),
        is_favorited: user.favorite_albums.contains(&album.id)
    };

    Ok(album_result)
}

pub fn add_album(conn: &mut DbConnection, new_album: NewAlbum) -> Result<NewAlbum, Error> {
    use crate::schema::albums::dsl::*;
    
    diesel::insert_into(albums)
        .values(&new_album)
        .execute(conn)
        .expect("Database error when inserting user");
    return Ok(new_album);
}
