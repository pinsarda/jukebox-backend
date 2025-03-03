use actix_web::{ get, post, web::{Data, Json, Query}, HttpResponse, Responder };
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::{fetcher::{ytmusic::YtMusicFetcher, Fetcher}, models::fetcher::FetcherQueryData, DbPool};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SearchQuery {
    pub query: String
}

#[utoipa::path()]
#[get("/fetcher/ytmusic/search")]
/// Get search results from Youtube Music
async fn yt_music_search(pool: Data<DbPool>, data: Query<SearchQuery>) -> impl Responder {
    YtMusicFetcher::new().search_musics(data.query.clone()).await;

    HttpResponse::Ok().body("Not implemented")
}

#[utoipa::path()]
#[post("/fetcher/ytmusic/add")]
/// Add a music from youtube music
async fn yt_music_add(pool: Data<DbPool>, data: Json<FetcherQueryData>) -> impl Responder {
    let conn = &mut pool.get().unwrap();

    YtMusicFetcher::new().add_music_with_album(conn, &data).await.unwrap();

    HttpResponse::Ok().body("Musique en cours d'ajout !")
}
