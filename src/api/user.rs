use actix_web::{ web::Data, Error };
use paperclip::actix::{ api_v2_operation, get, post, web::{self, Json} };
use crate::models::user::{ User, NewUser };
use crate::DbPool;
use crate::db_handlers::user::create_user;

#[api_v2_operation]
#[post("/user/signup")]
/// Signup a new user
async fn signup(pool: Data<DbPool>, new_user: Json<NewUser>) -> Result<Json<NewUser>, Error> {
    let conn = &mut pool.get().unwrap();
    
    let result = create_user(conn, new_user.into_inner()).unwrap();
    Ok(Json(result))
}