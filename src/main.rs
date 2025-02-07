mod api;
mod fetcher;
mod schema;
mod models;
mod db_handlers;
mod identity;

use actix_identity::IdentityMiddleware;
use actix_session::{ SessionMiddleware, storage::CookieSessionStore };
use actix_web::cookie::Key;
use actix_web::web::Data;
use actix_web::{ App, HttpServer, middleware::Logger };
use paperclip::actix::OpenApiExt;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

use api::{player::{ play, stop, next, previous, state }, routes::{ download, hello }};
use api::user::{ login, signup, get_info };

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DbConnection = SqliteConnection;


pub fn get_connection_pool() -> DbPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<DbConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let pool = get_connection_pool();
    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
           ))
            .wrap(Logger::default())
            .wrap_api()
            // api routes (to be removed)
            .service(hello)
            .service(download)
            // user managment
            .service(signup)
            .service(login)
            .service(get_info)
            // player api
            .service(play)
            .service(stop)
            .service(next)
            .service(previous)
            .service(state)
            .with_json_spec_at("/api/spec/v2")
            .with_swagger_ui_at("/swagger")
            .build()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}