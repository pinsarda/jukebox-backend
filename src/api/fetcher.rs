use actix_identity::Identity;
use actix_web::{ get, post, web::{Data, Json, Query}, HttpResponse, Responder };
use utoipa::ToSchema;
use crate::{fetcher::{youtube::YoutubeFetcher, ytmusic::YtMusicFetcher, Fetcher}, models::{fetcher::FetcherMusic, search::SearchQuery}, DbPool};

#[utoipa::path(
    request_body = SearchQuery,
    responses(
        (status = OK, body = Vec<FetcherMusic>),
        (status = FORBIDDEN)
    )
)]
#[get("/fetcher/ytmusic/search")]
/// Get search results from Youtube Music
async fn yt_music_search(_id: Identity, pool: Data<DbPool>, data: Query<SearchQuery>) -> impl Responder {
    let results = YtMusicFetcher::new().search_musics(data.query.clone()).await;

    HttpResponse::Ok().json(results)
}

#[utoipa::path(
    request_body = FetcherMusic,
    responses(
        (status = OK),
        (status = FORBIDDEN)
    )
)]
#[post("/fetcher/ytmusic/add")]
/// Add a music from youtube music
async fn yt_music_add(id: Identity, pool: Data<DbPool>, data: Json<FetcherMusic>) -> impl Responder {
    let conn = &mut pool.get().unwrap();
    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    match YtMusicFetcher::new().add_music_with_album(conn, &data, user_id).await {
        Ok(_) => HttpResponse::Ok().body("Succesfully added new music."),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[utoipa::path(
    request_body = SearchQuery,
    responses(
        (status = OK, body = Vec<FetcherMusic>),
        (status = FORBIDDEN)
    )
)]
#[get("/fetcher/youtube/search")]
/// Get search results from Youtube
async fn youtube_search(_id: Identity, pool: Data<DbPool>, data: Query<SearchQuery>) -> impl Responder {
    let results = YoutubeFetcher::new().search_musics(data.query.clone()).await;

    HttpResponse::Ok().json(results)
}

#[utoipa::path(
    request_body = FetcherMusic,
    responses(
        (status = OK),
        (status = FORBIDDEN)
    )
)]
#[post("/fetcher/youtube/add")]
/// Add a music from Youtube
async fn youtube_add(id: Identity, pool: Data<DbPool>, data: Json<FetcherMusic>) -> impl Responder {
    let conn = &mut pool.get().unwrap();
    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    match YoutubeFetcher::new().add_music_with_album(conn, &data, user_id).await {
        Ok(_) => HttpResponse::Ok().body("Succesfully added new music."),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
