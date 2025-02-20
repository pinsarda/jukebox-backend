use actix_web::{ web::Data, Error, HttpMessage, Result };
use diesel::result;
use paperclip::actix::{ api_v2_operation, get, post, web::{ Query, HttpRequest, Json } };
use crate::{ db_handlers::album::get_album_by_id, models::{album::{Album, AlbumResult, NewAlbum}, user::{ NewUser, User, UserData }}};
use crate::DbPool;
use crate::identity::UserIdentity;
use crate::models::Id;

#[api_v2_operation]
#[get("/album/metadata")]
/// Get an album metadata
async fn metadata(id: UserIdentity, pool: Data<DbPool>, query_data: Json<Id>) -> Result<Json<AlbumResult>, Error> {

    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let music = get_album_by_id(conn, query_data.id, user_id);

    Ok(Json(music.unwrap()))
}

#[api_v2_operation]
#[post("/album/add")]
/// Add an album to the database
async fn add_album(_id: UserIdentity, pool: Data<DbPool>, new_album: Json<NewAlbum>) -> Result<Json<NewAlbum>, Error> {

    let conn = &mut pool.get().unwrap();

    let result = crate::db_handlers::album::add_album(conn, new_album.into_inner()).unwrap();

    Ok(Json(result))
}
