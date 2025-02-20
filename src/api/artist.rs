use actix_web::{ web::Data, Error, HttpMessage, Result };
use diesel::result;
use paperclip::actix::{ api_v2_operation, get, post, web::{ Query, HttpRequest, Json } };
use crate::{ db_handlers::artist::get_artist_by_id, models::{artist::{Artist, ArtistResult, NewArtist}, user::{ NewUser, User, UserData }}};
use crate::DbPool;
use crate::identity::UserIdentity;
use crate::models::Id;

#[api_v2_operation]
#[get("/artist/metadata")]
/// Get an artist metadata
async fn metadata(id: UserIdentity, pool: Data<DbPool>, query_data: Json<Id>) -> Result<Json<ArtistResult>, Error> {

    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let music = get_artist_by_id(conn, query_data.id, user_id);

    Ok(Json(music.unwrap()))
}

#[api_v2_operation]
#[post("/artist/add")]
/// Add an artist to the database
async fn add_artist(_id: UserIdentity, pool: Data<DbPool>, new_artist: Json<NewArtist>) -> Result<Json<NewArtist>, Error> {

    let conn = &mut pool.get().unwrap();

    let result = crate::db_handlers::artist::add_artist(conn, new_artist.into_inner()).unwrap();

    Ok(Json(result))
}
