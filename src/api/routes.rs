use actix_web::{ get, post, web, HttpResponse, Responder };
use tokio::task;
use crate::fetcher::{download_video, play_audio};

#[utoipa::path()]
#[get("/helloworld")]
/// Displays a simple hello world
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[utoipa::path()]
#[get("/download/{id}")]
/// Downloads video with id id to the Downloads folder
async fn download(id: web::Path<String>) -> impl Responder {
    let url = "https://www.youtube.com/watch?v=".to_owned() + &id.into_inner();
    task::spawn(async {
        download_video(url).await;
        play_audio("test.m4a".to_string()).await;
    });
    HttpResponse::Ok().body("Download started !!!!")
}
