mod api;
mod downloader;

use actix_web::{ App, HttpServer };
use paperclip::actix::OpenApiExt;

use api::{player::{ play, stop, next, previous }, routes::{ download, hello }};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap_api()
            // api routes (to be removed)
            .service(hello)
            .service(download)
            // player api
            .service(play)
            .service(stop)
            .service(next)
            .service(previous)
            .with_json_spec_at("/api/spec/v2")
            .build()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}