mod api;
mod fetcher;
mod schema;
mod models;
mod db_handlers;
mod identity;

use std::error::Error;

use actix_identity::IdentityMiddleware;
use actix_session::{ SessionMiddleware, storage::CookieSessionStore };
use actix_web::cookie::Key;
use actix_web::web::Data;
use actix_web::{ App, HttpServer, middleware::Logger };
use diesel::pg::Pg;
use paperclip::actix::OpenApiExt;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use api::{player::{ play, stop, next, previous, state }, routes::{ download, hello }};
use api::user::{ login, signup, get_info };
use api::music::{self, add_music};
use api::album::{self, add_album};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PgConnection;
pub type DbBackend = Pg;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn run_migrations(connection: &mut impl MigrationHarness<DbBackend>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

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
    let mut connection = pool.get().expect("Failed to get connection from pool");

    run_migrations(&mut connection).expect("Error running migration");

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
            // music, album and artists managment
            .service(music::metadata)
            .service(add_music)
            .service(album::metadata)
            .service(add_album)
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
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}