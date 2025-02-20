use actix_identity::Identity;
use actix_web::web::Json;
use actix_web::HttpRequest;
use actix_web::{ web::Data, Error, HttpMessage, Result, post, get };
use crate::models::user::{ NewUser, User, UserData };
use crate::DbPool;
use crate::db_handlers::user::{ create_user, get_user_data, get_user };
use crate::identity::UserIdentity;


#[post("/user/signup")]
/// Signup a new user
async fn signup(pool: Data<DbPool>, new_user: Json<NewUser>) -> Result<Json<NewUser>, Error> {

    let conn = &mut pool.get().unwrap();
    let result = create_user(conn, new_user.into_inner()).unwrap();
    
    Ok(Json(result))
}


#[get("/user/get_info")]
/// Get user info
async fn get_info(id: UserIdentity, pool: Data<DbPool>) -> Result<Json<UserData>, Error> {

    let conn = &mut pool.get().unwrap();
    let user_data = get_user_data(conn, id.id().unwrap().parse::<i32>().unwrap());

    Ok(Json(user_data.unwrap()))
}


#[post("/user/login")]
/// Login existing user
async fn login(pool: Data<DbPool>, request: HttpRequest, new_user: Json<NewUser>) -> Result<Json<User>, Error> {

    let conn = &mut pool.get().unwrap();
    let user = get_user(conn, new_user.into_inner());

    let user_id = user.as_ref().unwrap().id.clone();

    Identity::login(&request.extensions(), user_id.to_string()).unwrap();
    Ok(Json(user.unwrap()))
}
