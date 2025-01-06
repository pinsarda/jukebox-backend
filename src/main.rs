mod api;
mod downloader;

use actix_web::{ App, HttpServer };
use api::routes::{ hello, download };

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