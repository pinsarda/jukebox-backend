use std::time::SystemTime;

use actix_identity::Identity;
use actix_web::{ get, post, web::{Data, Json, Query}, Error, HttpResponse, Responder, Result };
use crate::{ db_handlers::playlist::{get_playlist_by_id, to_rich_playlist}, models::playlist::{InsertablePlaylist, MusicAddRequest, NewPlaylist, Playlist, RichPlaylist}};
use crate::DbPool;
use crate::models::Id;


#[utoipa::path()]
#[get("/playlist/metadata")]
/// Get a playlist metadata
async fn metadata(id: Identity, pool: Data<DbPool>, query_data: Query<Id>) -> Result<Json<RichPlaylist>, Error> {

    let conn = &mut pool.get().unwrap();
    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let playlist = get_playlist_by_id(conn, query_data.id).unwrap();
    let rich_playlist = to_rich_playlist(conn, playlist, user_id).unwrap();

    Ok(Json(rich_playlist))
}

#[utoipa::path()]
#[post("/playlist/create")]
/// Create a playlist
async fn create_playlist(id: Identity, pool: Data<DbPool>, new_playlist: Json<NewPlaylist>) -> Result<Json<Playlist>, Error> {

    let conn = &mut pool.get().unwrap();
    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let insertable_playlist = InsertablePlaylist {
        title: new_playlist.title.clone(),
        owner_id: user_id,
        description: new_playlist.description.clone(),
        fetcher: new_playlist.fetcher.clone(),
        fetcher_id: new_playlist.fetcher_id.clone(),
        date_created: SystemTime::now()
    };

    let result = crate::db_handlers::playlist::create_playlist(conn, insertable_playlist).unwrap();

    Ok(Json(result))
}

#[utoipa::path()]
#[post("/playlist/add_music")]
/// Add a music to a playlist
async fn add_music(_id: Identity, pool: Data<DbPool>, query_data: Json<MusicAddRequest>) -> impl Responder {

    let conn = &mut pool.get().unwrap();

    crate::db_handlers::playlist::add_music_to_playlist(conn, query_data.playlist_id, query_data.music_id).unwrap();

    HttpResponse::Ok().body("Added music to playlist")
}

// #[post("/playlist/move_music")]
// /// Change the place of a music in the queue
// async fn move_music_in_playlist(_id: Identity, pool: Data<DbPool>, query_data: Json<MoveMusicInQueueRequest>) -> impl Responder {
//     move_music_in_playlist(query_data.old_index, query_data.new_index);
//     HttpResponse::Ok().body("Updated Queue succesfully")
// }