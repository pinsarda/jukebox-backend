use actix_identity::Identity;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse, Responder, http::StatusCode, web::Query};
use actix_web::{ web::Data, HttpMessage, Result, post, get };
use crate::models::user::{ Favorites, NewUser, UserData };
use crate::models::Id;
use crate::DbPool;
use crate::db_handlers::user::{ create_user, get_favorites, get_user, get_user_data, verify_password };


#[utoipa::path(
    request_body = NewUser,
    responses(
        (status = 200, description = "User created successfully"),
        (status = 400, description = "Username already exists")
    ),
    tag = "user"
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

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully retrieved user personal info", body = UserData),
        (status = 401, description = "Unauthorized access")
    ),
    tag = "user"
)]
#[get("/user/get_personal_info")]
/// Get requesting user info
async fn get_personal_info(id: Identity, pool: Data<DbPool>) -> Result<Json<UserData>> {

    let conn = &mut pool.get().unwrap();
    let user_data = get_user_data(conn, id.id().unwrap().parse::<i32>().unwrap());

    Ok(Json(user_data.unwrap()))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully retrieved user public info", body = UserData),
        (status = 404, description = "User not found")
    ),
    tag = "user"
)]
#[get("/user/get_info")]
/// Get user public info
async fn get_info(_id: Identity, pool: Data<DbPool>, query_data: Query<Id>) -> Result<Json<UserData>> {

    let conn = &mut pool.get().unwrap();
    let user_data = get_user_data(conn, query_data.id);

    Ok(Json(user_data.unwrap()))
}

#[utoipa::path(
    request_body = NewUser,
    responses(
        (status = 200, description = "User authenticated successfully"),
        (status = 400, description = "User doesn't exist or wrong password")
    ),
    tag = "user"
)]
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

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully retrieved user favorites", body = Favorites),
        (status = 401, description = "Unauthorized access")
    ),
    tag = "user"
)]
#[get("/user/favorites")]
/// Get user favorites (only works with musics for now)
async fn favorites(id: Identity, pool: Data<DbPool>) -> Result<Json<Favorites>> {

    let conn = &mut pool.get().unwrap();
    let user_id = id.id().unwrap().parse::<i32>().unwrap();

    let result = Favorites {
        artists: Vec::new(),
        albums: Vec::new(),
        musics: get_favorites(conn, user_id).unwrap()
    };

    Ok(Json(result))
}