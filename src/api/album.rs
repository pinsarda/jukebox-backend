use actix_identity::Identity;
use actix_web::{ get, post, web::{Data, Json, Query}, Error, Result };
use crate::{ db_handlers::album::{get_album_by_id, to_rich_album}, models::album::{Album, NewAlbum, RichAlbum}};
use crate::DbPool;
use crate::models::Id;


#[utoipa::path()]
#[get("/album/metadata")]
/// Get an album metadata
async fn metadata(id: Identity, pool: Data<DbPool>, query_data: Query<Id>) -> Result<Json<RichAlbum>, Error> {

    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let album = get_album_by_id(conn, query_data.id).unwrap();
    let rich_album = to_rich_album(conn, album, user_id).unwrap();

    Ok(Json(rich_album))
}

#[utoipa::path()]
#[post("/album/add")]
/// Add an album to the database
async fn add_album(id: Identity, pool: Data<DbPool>, new_album: Json<NewAlbum>) -> Result<Json<Album>, Error> {

    let conn = &mut pool.get().unwrap();
    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let mut new_album = new_album.into_inner();
    new_album.origin_user_id = user_id;
    new_album.fetcher = None;

    let result = crate::db_handlers::album::add_album(conn, new_album).unwrap();

    Ok(Json(result))
}
