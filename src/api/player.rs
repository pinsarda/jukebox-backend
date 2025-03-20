

use actix_identity::Identity;
use actix_web::{ get, post, web::{Data, Json}, HttpResponse, Responder };

use crate::{db_handlers::music::get_music_by_id, models::Id, player::Player, DbPool};

#[utoipa::path(
    request_body = Id,
    responses(
        (status = OK),
        (status = FORBIDDEN)
    )
)]
#[post("/player/add_to_queue")]
/// Add music to queue
async fn add_to_queue(_id: Identity, pool: Data<DbPool>, player: Data<Player>, query_data: Json<Id>) -> impl Responder {
    let conn = &mut pool.get().unwrap();
    
    let music = get_music_by_id(conn, query_data.id).unwrap();
    player.add_to_queue(music).await;

    HttpResponse::Ok().body("Added music to queue")
}

#[utoipa::path(
    responses(
        (status = OK),
        (status = FORBIDDEN)
    )
)]
#[post("/player/play")]
/// Start playback of enqueued music
async fn play(_id: Identity, player: Data<Player>) -> impl Responder {
    player.play();
    HttpResponse::Ok().body("Starting playback")
}

#[utoipa::path(
    responses(
        (status = OK),
        (status = FORBIDDEN)
    )
)]
#[post("/player/pause")]
/// Pause music playback
async fn pause(_id: Identity, player: Data<Player>) -> impl Responder {
    player.pause();
    HttpResponse::Ok().body("Pausing music")
}

#[utoipa::path(
    responses(
        (status = OK),
        (status = FORBIDDEN)
    )
)]
#[post("/player/stop")]
/// Stop music playback
async fn stop() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[utoipa::path(
    responses(
        (status = OK),
        (status = FORBIDDEN)
    )
)]
#[post("/player/next")]
/// Skip to next music in queue
async fn next(_id: Identity, player: Data<Player>) -> impl Responder {
    player.next();
    HttpResponse::Ok().body("Skipping to next song in queue")
}

#[utoipa::path(
    responses(
        (status = OK),
        (status = FORBIDDEN)
    )
)]
#[post("/player/previous")]
/// Skip to previous music in queue
async fn previous(_id: Identity, player: Data<Player>) -> impl Responder {
    player.previous();
    HttpResponse::Ok().body("Skipping to previous song in queue")
}

#[utoipa::path(
    responses(
        (status = OK),
        (status = FORBIDDEN)
    )
)]
#[post("/player/seek")]
/// Seek to 
async fn seek() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[utoipa::path(
    responses(
        (status = OK),
        (status = FORBIDDEN)
    )
)]
#[get("/player/state")]
/// Get player state
async fn state() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

// TODO : add /player/socket for live update on all clients