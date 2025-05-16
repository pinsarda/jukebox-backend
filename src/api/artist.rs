use actix_identity::Identity;
use actix_web::{ get, post, web::{Data, Json, Query}, Error, Result };
use crate::{ db_handlers::artist::{get_artist_by_id, to_rich_artist}, models::{album::RichAlbum, artist::{Artist, NewArtist, RichArtist}}};
use crate::DbPool;
use crate::models::Id;


#[utoipa::path(
    responses(
        (status = OK, description = "Successfully retrieved artist metadata", body = RichArtist),
        (status = NOT_FOUND, description = "Artist not found"),
        (status = UNAUTHORIZED, description = "Unauthorized access")
    ),
    tag = "artist"
)]
#[get("/artist/metadata")]
/// Get an artist metadata
async fn metadata(id: Identity, pool: Data<DbPool>, query_data: Query<Id>) -> Result<Json<RichArtist>, Error> {

    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let artist = get_artist_by_id(conn, query_data.id).unwrap();
    let rich_artist = to_rich_artist(conn, artist, user_id);

    Ok(Json(rich_artist.unwrap()))
}

#[utoipa::path(
    request_body = NewArtist,
    responses(
        (status = CREATED, description = "Artist successfully added", body = Artist),
        (status = BAD_REQUEST, description = "Invalid artist data provided"),
        (status = UNAUTHORIZED, description = "Unauthorized access")
    ),
    tag = "artist"
)]
#[post("/artist/add")]
/// Add an artist to the database
async fn add_artist(_id: Identity, pool: Data<DbPool>, new_artist: Json<NewArtist>) -> Result<Json<Artist>, Error> {

    let conn = &mut pool.get().unwrap();

    let result = crate::db_handlers::artist::add_artist(conn, new_artist.into_inner()).unwrap();

    Ok(Json(result))
}

#[utoipa::path(
    responses(
        (status = OK, description = "Successfully retrieved albums for the artist", body = Vec<RichAlbum>),
        (status = NOT_FOUND, description = "Artist not found"),
        (status = UNAUTHORIZED, description = "Unauthorized access")
    ),
    tag = "artist"
)]
#[get("/artist/get_albums")]
/// Get albums by an artist
async fn get_albums(id: Identity, pool: Data<DbPool>, query_data: Query<Id>) -> Result<Json<Vec<RichAlbum>>, Error> {

    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let result = crate::db_handlers::artist::get_albums_from_artist(conn, query_data.id, user_id).unwrap();

    Ok(Json(result))
}
