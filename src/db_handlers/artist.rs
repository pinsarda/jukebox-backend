use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::result::Error;
use crate::db_handlers::album::to_rich_album;
use crate::models::album::Album;
use diesel::sql_query;
use crate::models::album::RichAlbum;
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

pub fn get_artist_by_name(conn: &mut DbConnection, artist_name: String) -> Result<Artist, Error> {
    use crate::schema::artists::*;

    let artist_result = dsl::artists
        .filter(name.eq(artist_name))
        .select(Artist::as_select())
        .first(conn);

    artist_result
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

pub fn get_albums_from_artist(conn: &mut DbConnection, artist_id: i32, user_id: i32) -> Result<Vec<RichAlbum>, Error> {

    // Raw SQL is not great but couldn't figure another way to use = ANY() properly
    // (Diesel eq_any doesn't do quite that apparently)
    let albums_results = sql_query(
        format!("SELECT *
        FROM albums
        WHERE {} = ANY (artists_ids);", artist_id)
    ).load::<Album>(conn)
    .unwrap();

    print!("{:#?}", albums_results);

    let artist_albums: Vec<RichAlbum> = albums_results.into_iter().map(|album| {
            to_rich_album(conn, album, user_id).unwrap()
    }).collect();

    Ok(artist_albums)
}

pub fn add_artist(conn: &mut DbConnection, new_artist: NewArtist) -> Result<Artist, Error> {
    use crate::schema::artists::dsl::*;
    
    let inserted_artist = diesel::insert_into(artists)
        .values(&new_artist)
        .get_result::<Artist>(conn).unwrap();

    return Ok(inserted_artist);
}

pub async fn search_artists(conn: &mut DbConnection, query: &str, user_id: i32) -> Result<Vec<RichArtist>, Error> {
    use crate::schema::artists::dsl::*;
    
    // Temporary solution until proper fuzzy searching is implemented for postgres
    let mut pattern_query = "%".to_string();
    pattern_query.push_str(query);
    pattern_query.push_str("%");

    let search_result: Vec<Artist> = 
        artists
        .filter(name.ilike(pattern_query))
        .limit(5)
        .select(Artist::as_select())
        .load(conn)
        .expect("Error searching music");

    let results: Vec<RichArtist> = search_result.into_iter().map(|music| {
            to_rich_artist(conn, music, user_id).unwrap()
    }).collect();

    Ok(results)
}