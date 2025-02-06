use actix_web::{ HttpResponse, Responder };
use tokio::task;
use paperclip::actix::{ api_v2_operation, get, web::{ self, Json }};
use crate::fetcher::{download_video, play_audio};

#[api_v2_operation]
#[get("/helloworld")]
/// Displays a simple hello world
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[api_v2_operation]
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
