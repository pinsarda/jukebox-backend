use actix_web::{ post, web::{Data, Json}, HttpResponse, Responder };
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::{fetcher::{ytmusic::YtMusicFetcher, Fetcher}, DbPool};


#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct YtMusicMusicId {
    pub ytmusic_music_id: String
}

#[utoipa::path()]
#[post("/fetcher/ytmusic/add")]
/// Add the music with id id to 
async fn yt_music_add(pool: Data<DbPool>, data: Json<YtMusicMusicId>) -> impl Responder {
    let conn = &mut pool.get().unwrap();

    YtMusicFetcher::new().add_music(conn, &data.ytmusic_music_id).unwrap();

    HttpResponse::Ok().body("Musique en cours d'ajout !")
}
