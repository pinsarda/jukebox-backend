mod api;
mod downloader;

use actix_web::{ App, HttpServer };
use paperclip::actix::OpenApiExt;

use api::routes::{ hello, download };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap_api()
            .service(hello)
            .service(download)
            .with_json_spec_at("/api/spec/v2")
            .build()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}