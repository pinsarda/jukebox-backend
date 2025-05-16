use actix_identity::Identity;
use actix_web::{ get, web::{Data, Json, Query}, Error, Result };
use crate::models::{album::RichAlbum, artist::RichArtist, music::RichMusic, search::{SearchQuery, SearchResult}};
use crate::DbPool;

#[utoipa::path(
    request_body = SearchQuery,
    responses(
        (status = OK, description = "Successfully retrieved search results for musics", body = Vec<RichMusic>),
        (status = FORBIDDEN, description = "Access forbidden")
    ),
    tag = "search"
)]
#[get("/search_musics")]
/// Get search results of a query for musics
async fn search_musics(id: Identity, pool: Data<DbPool>, query_data: Query<SearchQuery>) -> Result<Json<Vec<RichMusic>>, Error> {

    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let result = crate::db_handlers::music::search_musics(conn, &query_data.query, user_id).await.expect("Error searching musics");

    Ok(Json(result))
}

#[utoipa::path(
    request_body = SearchQuery,
    responses(
        (status = OK, description = "Successfully retrieved search results for albums", body = Vec<RichAlbum>),
        (status = FORBIDDEN, description = "Access forbidden")
    ),
    tag = "search"
)]
#[get("/search_albums")]
/// Get search results of a query for albums
async fn search_albums(id: Identity, pool: Data<DbPool>, query_data: Query<SearchQuery>) -> Result<Json<Vec<RichAlbum>>, Error> {

    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let result = crate::db_handlers::album::search_albums(conn, &query_data.query, user_id).await.expect("Error searching musics");

    Ok(Json(result))
}

#[utoipa::path(
    request_body = SearchQuery,
    responses(
        (status = OK, description = "Successfully retrieved search results for artists", body = Vec<RichArtist>),
        (status = FORBIDDEN, description = "Access forbidden")
    ),
    tag = "search"
)]
#[get("/search_artists")]
/// Get search results of a query for artists
async fn search_artists(id: Identity, pool: Data<DbPool>, query_data: Query<SearchQuery>) -> Result<Json<Vec<RichArtist>>, Error> {

    let conn = &mut pool.get().unwrap();

    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let result = crate::db_handlers::artist::search_artists(conn, &query_data.query, user_id).await.expect("Error searching musics");

    Ok(Json(result))
}

#[utoipa::path(
    request_body = SearchQuery,
    responses(
        (status = OK, description = "Successfully retrieved comprehensive search results", body = SearchResult),
        (status = FORBIDDEN, description = "Access forbidden")
    ),
    tag = "search"
)]
#[get("/search")]
/// Get comprehensive search results of a query
async fn search(id: Identity, pool: Data<DbPool>, query_data: Query<SearchQuery>) -> Result<Json<SearchResult>, Error> {
    let user_id = id.id().unwrap().parse::<i32>().unwrap();
    let query = query_data.query.to_string();

    let music_conn = &mut pool.get().unwrap();
    let album_conn = &mut pool.get().unwrap();
    let artist_conn = &mut pool.get().unwrap();

    let (musics_result, albums_results, artists_result) = tokio::join!(
        crate::db_handlers::music::search_musics(music_conn, &query, user_id),
        crate::db_handlers::album::search_albums(album_conn, &query, user_id),
        crate::db_handlers::artist::search_artists(artist_conn, &query, user_id)
    );

    let musics = musics_result.unwrap();
    let albums = albums_results.unwrap();
    let artists = artists_result.unwrap();

    Ok(Json(SearchResult {
        musics: musics,
        albums: albums,
        artists: artists
    }))
}