use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::result::Error;
use crate::models::artist::{ Artist, NewArtist, ArtistResult };
use crate::DbConnection;
use crate::db_handlers::user::get_user_by_id;

pub fn get_artist_by_id(conn: &mut DbConnection, artist_id: i32, user_id: i32) -> Result<ArtistResult, Error> {
    use crate::schema::artists::dsl::artists;

    let artist = artists
        .find(artist_id)
        .select(Artist::as_select())
        .first(conn)
        .expect("Error getting artist by id");

    let user = get_user_by_id(conn, user_id).expect("Error while getting user data");

    let artist_result = ArtistResult {
        id: artist.id,
        name: artist.name,
        albums: vec![],
        is_favorited: user.favorite_artists.contains(&artist_id)
    };

    Ok(artist_result)
    
}

pub fn add_artist(conn: &mut DbConnection, new_artist: NewArtist) -> Result<NewArtist, Error> {
    use crate::schema::artists::dsl::*;
    
    diesel::insert_into(artists)
        .values(&new_artist)
        .execute(conn)
        .expect("Database error when inserting artist");
    return Ok(new_artist);
}
