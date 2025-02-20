use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::result::Error;
use crate::models::album::{ Album, NewAlbum, AlbumResult };
use crate::DbConnection;
use crate::db_handlers::user::get_user_by_id;

pub fn get_album_by_id(conn: &mut DbConnection, album_id: i32, user_id: i32) -> Result<AlbumResult, Error> {
    use crate::schema::albums::dsl::albums;

    let album = albums
        .find(album_id)
        .select(Album::as_select())
        .first(conn)
        .expect("Error getting album_by_id");

    let user = get_user_by_id(conn, user_id).expect("Error while getting user data");

    let album_result = AlbumResult {
        id: album.id,
        title: album.title,
        artists_ids: album.artists_ids,
        musics: vec![],
        is_favorited: user.favorite_albums.contains(&album_id)
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
