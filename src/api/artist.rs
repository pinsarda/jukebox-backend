use actix_identity::Identity;
use actix_web::{ get, post, web::{Data, Json, Query}, Error, HttpResponse, Responder, Result };
use crate::{ db_handlers::artist::{get_artist_by_id, to_rich_artist}, models::{album::RichAlbum, artist::{Artist, NewArtist, RichArtist}}};
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
async fn add_artist(_id: Identity, pool: Data<DbPool>, new_artist: Json<NewArtist>) -> Result<Json<Artist>, Error> {

    let conn = &mut pool.get().unwrap();

    let result = crate::db_handlers::artist::add_artist(conn, new_artist.into_inner()).unwrap();

    Ok(Json(result))
}

#[utoipa::path()]
#[get("/artist/get_albums")]
/// Add an artist to the database
async fn get_albums(id: Identity, pool: Data<DbPool>, query_data: Query<Id>) -> Result<Json<Vec<RichAlbum>>, Error> {

    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let result = crate::db_handlers::artist::get_albums_from_artist(conn, query_data.id, user_id).unwrap();

    Ok(Json(result))
}

#[utoipa::path()]
#[post("/artist/add_favorite")]
/// TODO : Add an artist to the fav db
async fn add_favorite_artist(id: Identity, pool: Data<DbPool>, query_data: Json<Id>) -> impl Responder {
    
    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let result = crate::db_handlers::user::add_favorite_artist(conn, user_id, query_data.id);

    match result {
        Ok(_) => HttpResponse::Ok().body("Artist added to favorites."),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }    
}

#[utoipa::path()]
#[post("/artist/remove_favorite")]
/// TODO : remove artist from favorites
async fn remove_favorite_artist(id: Identity, pool: Data<DbPool>, query_data: Json<Id>) -> impl Responder {
    
    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let result = crate::db_handlers::user::remove_favorite_artist(conn, user_id, query_data.id);

    match result {
        Ok(_) => HttpResponse::Ok().body("Artist removed from favorites."),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }    
}
