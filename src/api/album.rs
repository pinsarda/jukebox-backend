use actix_identity::Identity;
use actix_web::{ get, post, web::{Data, Json, Query}, Error, HttpResponse, Responder, Result };
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

#[utoipa::path()]
#[post("/album/add_favorite")]
/// TODO : Add an album to the fav db
async fn add_favorite_album(id: Identity, pool: Data<DbPool>, query_data: Json<Id>) -> impl Responder {
    
    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let result = crate::db_handlers::user::add_favorite_album(conn, user_id, query_data.id);

    match result {
        Ok(_) => HttpResponse::Ok().body("Album added to favorites."),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }    
}

#[utoipa::path()]
#[post("/album/remove_favorite")]
/// TODO : remove album from favorites
async fn remove_favorite_album(id: Identity, pool: Data<DbPool>, query_data: Json<Id>) -> impl Responder {
    
    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let result = crate::db_handlers::user::remove_favorite_album(conn, user_id, query_data.id);

    match result {
        Ok(_) => HttpResponse::Ok().body("Album removed from favorites."),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }    
}

