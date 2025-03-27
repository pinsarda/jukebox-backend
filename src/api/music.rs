use actix_identity::Identity;
use actix_web::{ get, post, web::{Data, Json, Query}, Error, HttpResponse, Responder, Result };
use crate::{db_handlers::music::{get_music_by_id, to_rich_music}, models::music::{RichMusic, NewMusic}};
use crate::DbPool;
use crate::models::Id;

#[utoipa::path()]
#[get("/music/metadata")]
/// Get music metadata
async fn metadata(id: Identity, pool: Data<DbPool>, query_data: Query<Id>) -> Result<Json<RichMusic>, Error> {

    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let music = get_music_by_id(conn, query_data.id).unwrap();
    let rich_music = to_rich_music(conn, music, user_id);

    Ok(Json(rich_music.unwrap()))
}

#[utoipa::path()]
#[post("/music/add")]
/// Add a music to the database
async fn add_music(_id: Identity, pool: Data<DbPool>, new_music: Json<NewMusic>) -> Result<Json<NewMusic>, Error> {
    
    let conn = &mut pool.get().unwrap();

    let result = crate::db_handlers::music::add_music(conn, new_music.into_inner()).unwrap();

    Ok(Json(result))
}

#[utoipa::path()]
#[post("/music/add_favorite")]
/// Add a music to the database
async fn add_favorite_music(id: Identity, pool: Data<DbPool>, query_data: Json<Id>) -> impl Responder {
    
    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let result = crate::db_handlers::user::add_favorite_music(conn, user_id, query_data.id);

    match result {
        Ok(_) => HttpResponse::Ok().body("Favorited music"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }    
}

#[utoipa::path()]
#[post("/music/remove_favorite")]
/// Add a music to the database
async fn remove_favorite_music(id: Identity, pool: Data<DbPool>, query_data: Json<Id>) -> impl Responder {
    
    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let result = crate::db_handlers::user::remove_favorite_music(conn, user_id, query_data.id);

    match result {
        Ok(_) => HttpResponse::Ok().body("Removed favorite for music"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }    
}
