use std::string;

use youtube_dl::YoutubeDl;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/test")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("aqqqq world!")
}

#[get("/download/{id}")]
async fn download(id: web::Path<String>) -> impl Responder {
    let url = "https://www.youtube.com/watch?v=".to_owned() + &id.into_inner();
    let output = YoutubeDl::new(&url)
    .socket_timeout("15")
    .download_to("Downloads");
    HttpResponse::Ok().body("Download successful !!!!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(download)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}