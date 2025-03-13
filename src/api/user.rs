use actix_identity::Identity;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse, Responder, http::StatusCode};
use actix_web::{ web::Data, HttpMessage, Result, post, get };
use crate::models::user::{ NewUser, UserData };
use crate::DbPool;
use crate::db_handlers::user::{ create_user, get_user, get_user_data, verify_password };


#[utoipa::path(
    responses(
        (status = 200, description = "User created succesfully"),
        (status = 400, description = "Username already exists")
    ),
)]
#[post("/user/signup")]
/// Signup a new user
async fn signup(pool: Data<DbPool>, new_user: Json<NewUser>) -> impl Responder {

    let conn = &mut pool.get().unwrap();
    let result = create_user(conn, new_user.into_inner());
    
    match result {
        Ok(_) => HttpResponse::build(StatusCode::OK).json("User created succesfully"),
        Err(_) => HttpResponse::build(StatusCode::BAD_REQUEST).json("Username already exists"),
    }
    
}

#[utoipa::path()]
#[get("/user/get_info")]
/// Get user info
async fn get_info(id: Identity, pool: Data<DbPool>) -> Result<Json<UserData>> {

    let conn = &mut pool.get().unwrap();
    let user_data = get_user_data(conn, id.id().unwrap().parse::<i32>().unwrap());

    Ok(Json(user_data.unwrap()))
}

#[utoipa::path()]
#[post("/user/login")]
/// Login existing user
async fn login(pool: Data<DbPool>, request: HttpRequest, new_user: Json<NewUser>) -> impl Responder {
    let new_user_password = new_user.password.clone();

    let conn = &mut pool.get().unwrap();
    let user_result = get_user(conn, new_user.into_inner());

    let user = match user_result {
        Ok(user) => user,
        Err(_) => return HttpResponse::build(StatusCode::BAD_REQUEST).json("User doesn't exist"),
    }; 

    if verify_password(user.password, new_user_password).unwrap() {
        let user_id = user.id;
        Identity::login(&request.extensions(), user_id.to_string()).unwrap();
        HttpResponse::build(StatusCode::OK).json("User authenticated succesfully")
    } else {
        HttpResponse::build(StatusCode::BAD_REQUEST).json("Wrong password")
    }
}
