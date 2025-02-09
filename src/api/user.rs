use actix_identity::Identity;
use actix_web::{ web::Data, Error, HttpMessage, Result };
use paperclip::actix::{ api_v2_operation, post, web::{ Json, HttpRequest } };
use crate::models::user::{ NewUser, User };
use crate::DbPool;
use crate::db_handlers::user::{ create_user, get_user_by_id, get_user };
use crate::identity::UserIdentity;

#[api_v2_operation]
#[post("/user/signup")]
/// Signup a new user
async fn signup(pool: Data<DbPool>, new_user: Json<NewUser>) -> Result<Json<NewUser>, Error> {

    let conn = &mut pool.get().unwrap();
    let result = create_user(conn, new_user.into_inner()).unwrap();
    
    Ok(Json(result))
}

#[api_v2_operation]
#[post("/user/get_info")]
/// Get user info
async fn get_info(id: UserIdentity, pool: Data<DbPool>) -> Result<String, Error> {

    let conn = &mut pool.get().unwrap();
    let user = get_user_by_id(conn, id.id().unwrap().parse::<i32>().unwrap()).unwrap();

    Ok(format!("Welcome! {}", user.username))
}

#[api_v2_operation]
#[post("/user/login")]
/// Login existing user
async fn login(pool: Data<DbPool>, request: HttpRequest, new_user: Json<NewUser>) -> Result<Json<User>, Error> {

    let conn = &mut pool.get().unwrap();
    let user = get_user(conn, new_user.into_inner());

    let user_id = user.as_ref().unwrap().id.clone();

    Identity::login(&request.extensions(), user_id.to_string()).unwrap();
    Ok(Json(user.unwrap()))
}
