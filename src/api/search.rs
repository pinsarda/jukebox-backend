use actix_identity::Identity;
use actix_web::{ get, web::{Data, Json, Query}, Error, Result };
use crate::models::{music::RichMusic, SearchQuery};
use crate::DbPool;

#[utoipa::path()]
#[get("/search_musics")]
/// Get search results of a query
async fn search_musics(id: Identity, pool: Data<DbPool>, query_data: Query<SearchQuery>) -> Result<Json<Vec<RichMusic>>, Error> {

    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let result = crate::db_handlers::music::search_musics(conn, &query_data.query, user_id).expect("Error searching musics");

    Ok(Json(result))
}