use std::{sync::Mutex, time::Duration};
use actix_ws::Session;
use futures_util::StreamExt as _;
use actix_identity::Identity;
use actix_web::{ get, Error, post, web::{self, Data, Json}, HttpRequest, HttpResponse, Responder };

use crate::{db_handlers::{analytics::log_playback, music::{get_music_by_id, to_rich_music}}, models::{player::{MoveMusicInQueueRequest, RichPlayerState, SeekRequest, VolumeChangeRequest}, Id, Index}, player::Player, DbPool};

#[utoipa::path(
    request_body = Id,
    responses(
        (status = OK),
        (status = FORBIDDEN)
    ),
    tag = "player"
)]
#[post("/player/add_to_queue")]
/// Add music to queue
async fn add_to_queue(id: Identity, pool: Data<DbPool>, player: Data<Player>, query_data: Json<Id>, socket_sessions: Data<Mutex<Vec<Session>>>) -> impl Responder {
    let conn = &mut pool.get().unwrap();
    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let music = get_music_by_id(conn, query_data.id).unwrap();
    log_playback(conn, music.id, music.album_id, user_id).unwrap();
    player.add_to_queue(music).await;

    notify_sessions(socket_sessions, String::from("adding to queue")).await;

    HttpResponse::Ok().body("Added music to queue")
}

#[utoipa::path(
    responses(
        (status = OK),
        (status = FORBIDDEN)
    ),
    tag = "player"
)]
#[post("/player/play")]
/// Start playback of enqueued music
async fn play(_id: Identity, player: Data<Player>, socket_sessions: Data<Mutex<Vec<Session>>>) -> impl Responder {
    player.play();
    notify_sessions(socket_sessions, String::from("playing")).await;
    HttpResponse::Ok().body("Starting playback")
}

#[utoipa::path(
    responses(
        (status = OK),
        (status = FORBIDDEN)
    ),
    tag = "player"
)]
#[post("/player/pause")]
/// Pause music playback
async fn pause(_id: Identity, player: Data<Player>, socket_sessions: Data<Mutex<Vec<Session>>>) -> impl Responder {
    player.pause();
    notify_sessions(socket_sessions, String::from("pausing")).await;
    HttpResponse::Ok().body("Pausing music")
}

#[utoipa::path(
    responses(
        (status = OK),
        (status = FORBIDDEN)
    ),
    tag = "player"
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
    ),
    tag = "player"
)]
#[post("/player/next")]
/// Skip to next music in queue
async fn next(_id: Identity, player: Data<Player>, socket_sessions: Data<Mutex<Vec<Session>>>) -> impl Responder {
    player.next();
    notify_sessions(socket_sessions, String::from("next")).await;
    HttpResponse::Ok().body("Skipping to next song in queue")
}

#[utoipa::path(
    responses(
        (status = OK),
        (status = FORBIDDEN)
    ),
    tag = "player"
)]
#[post("/player/previous")]
/// Skip to previous music in queue
async fn previous(_id: Identity, player: Data<Player>, socket_sessions: Data<Mutex<Vec<Session>>>) -> impl Responder {
    player.previous();
    notify_sessions(socket_sessions, String::from("previous")).await;
    HttpResponse::Ok().body("Skipping to previous song in queue")
}

#[utoipa::path(
    responses(
        (status = OK, body=SeekRequest),
        (status = FORBIDDEN)
    ),
    tag = "player"
)]
#[post("/player/seek")]
/// Seek to a time in the currently playing music
async fn seek(_id: Identity, player: Data<Player>, socket_sessions: Data<Mutex<Vec<Session>>>, query_data: Json<SeekRequest>) -> impl Responder {
    player.seek(Duration::from_millis(query_data.pos));
    notify_sessions(socket_sessions, String::from("seeking")).await;
    HttpResponse::Ok().body("Seeked succesfully")
}

#[utoipa::path(
    responses(
        (status = OK, body=VolumeChangeRequest),
        (status = FORBIDDEN)
    ),
    tag = "player"
)]
#[post("/player/set_volume")]
/// Set music volume
async fn set_volume(_id: Identity, player: Data<Player>, socket_sessions: Data<Mutex<Vec<Session>>>, query_data: Json<VolumeChangeRequest>) -> impl Responder {
    player.set_volume(query_data.volume);
    notify_sessions(socket_sessions, String::from("volume adjustment")).await;
    HttpResponse::Ok().body("Changed volume succesfully")
}

#[utoipa::path(
    responses(
        (status = OK, body=MoveMusicInQueueRequest),
        (status = FORBIDDEN)
    ),
    tag = "player"
)]
#[post("/player/move_music_in_queue")]
/// Change the place of a music in the queue
async fn move_music_in_queue(_id: Identity, player: Data<Player>, socket_sessions: Data<Mutex<Vec<Session>>>, query_data: Json<MoveMusicInQueueRequest>) -> impl Responder {
    player.move_music_in_queue(query_data.old_index, query_data.new_index);
    notify_sessions(socket_sessions, String::from("queue update")).await;
    HttpResponse::Ok().body("Updated Queue succesfully")
}

#[utoipa::path(
    responses(
        (status = OK, body=Index),
        (status = FORBIDDEN)
    ),
    tag = "player"
)]
#[post("/player/move_in_queue")]
/// Plays the music at the given index in the queue
async fn move_in_queue(_id: Identity, player: Data<Player>, socket_sessions: Data<Mutex<Vec<Session>>>, query_data: Json<Index>) -> impl Responder {
    player.edit_queue_index(query_data.index);
    notify_sessions(socket_sessions, String::from("queue update")).await;
    HttpResponse::Ok().body("Updated Queue index succesfully")
}


#[utoipa::path(
    responses(
        (status = OK),
        (status = FORBIDDEN)
    ),
    tag = "player"
)]
#[post("/player/clear_queue")]
/// Clear the queue
async fn clear_queue(_id: Identity, player: Data<Player>, socket_sessions: Data<Mutex<Vec<Session>>>) -> impl Responder {
    player.clear_queue();
    notify_sessions(socket_sessions, String::from("queue update")).await;
    HttpResponse::Ok().body("Updated Queue succesfully")
}

#[utoipa::path(
    responses(
        (status = OK, body=RichPlayerState),
        (status = FORBIDDEN)
    ),
    tag = "player"
)]
#[get("/player/state")]
/// Get player state
async fn state(id: Identity, pool: Data<DbPool>, player: Data<Player>) -> Result<Json<RichPlayerState>, Error> {
    let player_state = player.get_state();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();
    let conn = &mut pool.get().unwrap();

    let rich_player_state = RichPlayerState {
        queue: player_state.queue.into_iter().map(|music| {
                to_rich_music(conn, music, user_id).unwrap()
            }).collect(),
        current_pos: player_state.current_pos,
        queue_index: player_state.queue_index,
        volume: player_state.volume,
        is_playing: player_state.is_playing
    };

    Ok(Json(rich_player_state))
}

#[utoipa::path(tag = "socket")]
#[get("/player/socket")]
/// Connect to the shared state websocket, allows to be notified when a change is made to the playback state
async fn socket(req: HttpRequest, stream: web::Payload, socket_sessions: Data<Mutex<Vec<Session>>>) -> Result<HttpResponse, Error> {
    let (res, session, _stream) = actix_ws::handle(&req, stream)?;

    socket_sessions.lock().unwrap().push(session.clone());
    
    Ok(res)
}

async fn notify_sessions(socket_sessions: Data<Mutex<Vec<Session>>>, message: String) {
    match socket_sessions.lock() {
        Ok(sessions) => for session in sessions.iter() {
            match session.clone().text(message.clone()).await {
                Ok(_) => (),
                Err(_) => () // Socket is closed and shall be removed from socket_sessions 
                // TODO alongside making socket_sessions an hashmap
            }
        },
        Err(err) => print!("Error in session {:#?}", err.to_string())
    }
}