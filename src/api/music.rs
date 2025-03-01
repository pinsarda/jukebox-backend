use actix_identity::Identity;
use actix_web::{ get, post, web::{Data, Json, Query}, Error, Result };
use crate::{db_handlers::music::{get_music_by_id, to_rich_music}, models::music::{RichMusic, NewMusic}};
use crate::DbPool;
use crate::models::Id;

#[utoipa::path()]
#[get("/music/metadata")]
/// Get music metadata
async fn metadata(id: Identity, pool: Data<DbPool>, query_data: Query<Id>) -> Result<Json<RichMusic>, Error> {

    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let music = get_music_by_id(conn, query_data.id).unwrap();
    let rich_music = to_rich_music(conn, music, user_id);

    Ok(Json(rich_music.unwrap()))
}

#[utoipa::path()]
#[post("/music/add")]
/// Add a music to the database
async fn add_music(_id: Identity, pool: Data<DbPool>, new_music: Json<NewMusic>) -> Result<Json<NewMusic>, Error> {
    
    let conn = &mut pool.get().unwrap();

    let result = crate::db_handlers::music::add_music(conn, new_music.into_inner()).unwrap();

    Ok(Json(result))
}
