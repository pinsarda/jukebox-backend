use actix_web::{ get, post, web::{Data, Json, Query}, HttpResponse, Responder };
use utoipa::ToSchema;
use crate::{fetcher::{ytmusic::YtMusicFetcher, Fetcher}, models::{fetcher::FetcherQueryData, SearchQuery}, DbPool};

#[utoipa::path()]
#[get("/fetcher/ytmusic/search")]
/// Get search results from Youtube Music
async fn yt_music_search(pool: Data<DbPool>, data: Query<SearchQuery>) -> impl Responder {
    let results = YtMusicFetcher::new().search(data.query.clone()).await;

    HttpResponse::Ok().json(results)
}

#[utoipa::path()]
#[post("/fetcher/ytmusic/add")]
/// Add a music from youtube music
async fn yt_music_add(pool: Data<DbPool>, data: Json<FetcherQueryData>) -> impl Responder {
    let conn = &mut pool.get().unwrap();

    match YtMusicFetcher::new().add_music_with_album(conn, &data).await {
        Ok(_) => HttpResponse::Ok().body("Succesfully added new music."),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
