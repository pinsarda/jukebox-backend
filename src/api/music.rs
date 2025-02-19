use actix_identity::Identity;
use actix_web::{ web::Data, Error, HttpMessage, Result };
use diesel::result;
use paperclip::actix::{ api_v2_operation, get, post, web::{ Query, HttpRequest, Json } };
use crate::{api::{music, user}, db_handlers::music::get_music_by_id, models::{music::{Music, MusicResult, NewMusic}, user::{ NewUser, User, UserData }}};
use crate::DbPool;
use crate::identity::UserIdentity;
use crate::models::Id;

#[api_v2_operation]
#[get("/music/metadata")]
/// Get music metadata
async fn metadata(id: UserIdentity, pool: Data<DbPool>, query_data: Json<Id>) -> Result<Json<MusicResult>, Error> {

    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let music = get_music_by_id(conn, query_data.id, user_id);

    Ok(Json(music.unwrap()))
}

#[api_v2_operation]
#[post("/music/add")]
/// Add a music to the database
async fn add_music(_id: UserIdentity, pool: Data<DbPool>, new_music: Json<NewMusic>) -> Result<Json<NewMusic>, Error> {
    use crate::schema::musics::dsl::musics;

    let conn = &mut pool.get().unwrap();

    let result = crate::db_handlers::music::add_music(conn, new_music.into_inner()).unwrap();

    Ok(Json(result))
}
