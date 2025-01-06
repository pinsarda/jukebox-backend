use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tokio::task;
use crate::downloader::{ download_video };

#[get("/test")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/download/{id}")]
async fn download(id: web::Path<String>) -> impl Responder {
    let url = "https://www.youtube.com/watch?v=".to_owned() + &id.into_inner();
    task::spawn(download_video(url));
    HttpResponse::Ok().body("Download started !!!!")
}
