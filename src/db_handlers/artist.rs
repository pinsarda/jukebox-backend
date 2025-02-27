use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::result::Error;
use crate::models::artist::{ Artist, NewArtist, RichArtist };
use crate::DbConnection;
use crate::db_handlers::user::get_user_by_id;


pub fn get_artist_by_id(conn: &mut DbConnection, artist_id: i32) -> Result<Artist, Error> {
    use crate::schema::artists::dsl::artists;

    let artist = artists
        .find(artist_id)
        .select(Artist::as_select())
        .first(conn)
        .expect("Error getting artist by id");

    Ok(artist)
}

pub fn to_rich_artist(conn: &mut DbConnection, artist: Artist, user_id: i32) -> Result<RichArtist, Error> {
    
    let user = get_user_by_id(conn, user_id).expect("Error while getting user data");

    let rich_artist = RichArtist {
        id: artist.id,
        name: artist.name,
        description: artist.description,
        is_favorited: user.favorite_artists.contains(&artist.id)
    };

    Ok(rich_artist)
}

pub fn get_artists_by_ids(conn: &mut DbConnection, artists_ids: Vec<i32>, user_id: i32) -> Result<Vec<RichArtist>, Error> {
    use crate::schema::artists;

    let artists = artists::table
        .filter(artists::id.eq_any(artists_ids))
        .select(Artist::as_select())
        .load::<Artist>(conn)
        .expect("Error getting artists");

    let results: Vec<RichArtist> = artists.into_iter().map(|music| {
        to_rich_artist(conn, music, user_id).unwrap()
    }).collect();

    Ok(results)
}


pub fn add_artist(conn: &mut DbConnection, new_artist: NewArtist) -> Result<NewArtist, Error> {
    use crate::schema::artists::dsl::*;
    
    diesel::insert_into(artists)
        .values(&new_artist)
        .execute(conn)
        .expect("Database error when inserting artist");
    return Ok(new_artist);
}
