mod api;
mod fetcher;

use actix_web::{ App, HttpServer, middleware::Logger };
use paperclip::actix::OpenApiExt;

use api::{player::{ play, stop, next, previous, state }, routes::{ download, hello }};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap_api()
            // api routes (to be removed)
            .service(hello)
            .service(download)
            // player api
            .service(play)
            .service(stop)
            .service(next)
            .service(previous)
            .service(state)
            .with_json_spec_at("/api/spec/v2")
            .build()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}