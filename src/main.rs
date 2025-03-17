mod api;
mod fetcher;
mod schema;
mod models;
mod db_handlers;
mod downloader;
mod player;

use std::error::Error;

use actix_identity::IdentityMiddleware;
use actix_session::config::{CookieContentSecurity, PersistentSession};
use actix_session::{ SessionMiddleware, storage::CookieSessionStore };
use actix_web::cookie::time::Duration;
use actix_web::cookie::Key;
use actix_web::web::Data;
use actix_web::{ App, HttpServer, middleware::Logger };
use api::fetcher::{yt_music_add, yt_music_search};
use api::player::{add_to_queue, pause};
use api::search::{search, search_albums, search_artists, search_musics};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use api::{player::{ play, stop, next, previous, state }};
use api::user::{ login, signup, get_info };
use api::music::{self, add_music};
use api::album::{self, add_album};
use api::artist::{self, add_artist};
use rodio::OutputStream;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

use crate::player::Player;

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

    // Player should only be initialized once on startup
    // If _stream is dropped, the playback stops
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let player = Player::new(stream_handle);

    let mut connection = pool.get().expect("Failed to get connection from pool");

    run_migrations(&mut connection).expect("Error running migration");

    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(player.clone()))
            .wrap(IdentityMiddleware::builder()
                .login_deadline(Some(std::time::Duration::from_secs(60 * 60 * 24 * 365)))
                .build()
            )
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    secret_key.clone(),
                )
                .session_lifecycle(
                    PersistentSession::default().session_ttl(Duration::seconds(60 * 60 * 24 * 365))
                )
                .cookie_content_security(CookieContentSecurity::Private)
                .build()
            )
            .wrap(Logger::default())
            .into_utoipa_app()
            // user managment
            .service(signup)
            .service(login)
            .service(get_info)
            // music, album and artists managment
            .service(music::metadata)
            .service(add_music)
            .service(album::metadata)
            .service(add_album)
            .service(artist::metadata)
            .service(add_artist)
            // database search
            .service(search_musics) 
            .service(search_albums) 
            .service(search_artists) 
            .service(search) 
            // fetching
            .service(yt_music_add)
            .service(yt_music_search)
            // player api
            .service(add_to_queue)
            .service(play)
            .service(pause)
            .service(stop)
            .service(next)
            .service(previous)
            .service(state)
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api/openapi.json", api)
            })
            .into_app()
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}