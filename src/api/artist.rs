use actix_identity::Identity;
use actix_web::{ get, post, web::{Data, Json, Query}, Error, Result };
use crate::{ db_handlers::artist::{get_artist_by_id, to_rich_artist}, models::artist::{RichArtist, NewArtist}};
use crate::DbPool;
use crate::models::Id;


#[utoipa::path()]
#[get("/artist/metadata")]
/// Get an artist metadata
async fn metadata(id: Identity, pool: Data<DbPool>, query_data: Query<Id>) -> Result<Json<RichArtist>, Error> {

    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let artist = get_artist_by_id(conn, query_data.id).unwrap();
    let rich_artist = to_rich_artist(conn, artist, user_id);

    Ok(Json(rich_artist.unwrap()))
}

#[utoipa::path()]
#[post("/artist/add")]
/// Add an artist to the database
async fn add_artist(_id: Identity, pool: Data<DbPool>, new_artist: Json<NewArtist>) -> Result<Json<NewArtist>, Error> {

    let conn = &mut pool.get().unwrap();

    let result = crate::db_handlers::artist::add_artist(conn, new_artist.into_inner()).unwrap();

    Ok(Json(result))
}
