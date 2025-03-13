use actix_identity::Identity;
use actix_web::{ get, post, web::{Data, Json}, HttpResponse, Responder };

use crate::{api::music, db_handlers::music::get_music_by_id, downloader::{download, is_music_downloaded}, models::Id, DbPool};

#[utoipa::path()]
#[post("/player/play")]
/// Start playback of enqueued music
async fn play(id: Identity, pool: Data<DbPool>, query_data: Json<Id>) -> impl Responder {

    let conn = &mut pool.get().unwrap();
    
    let music = get_music_by_id(conn, query_data.id).unwrap();

    if !is_music_downloaded(&music) {
        download(&music).await;
    }

    // TODO play music file

    HttpResponse::Ok().body("Playing music")
}

#[utoipa::path()]
#[post("/player/stop")]
/// Stop music playback
async fn stop() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[utoipa::path()]
#[post("/player/next")]
/// Skip to next music in queue
async fn next() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[utoipa::path()]
#[post("/player/previous")]
/// Skip to previous music in queue
async fn previous() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[utoipa::path()]
#[post("/player/seek")]
/// Seek to 
async fn seek() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[utoipa::path()]
#[get("/player/state")]
/// Get player state
async fn state() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

// TODO : add /player/socket for live update on all clients