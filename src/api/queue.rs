use actix_web::{ HttpResponse, Responder};
use paperclip::actix::{ api_v2_operation, get, post, web::{ self, Json }};

#[api_v2_operation]
#[get("/queue/state")]
/// Get state of the queue
async fn state() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[api_v2_operation]
#[post("/queue/add/{music_id}")]
/// Enqueue a music
async fn add() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[api_v2_operation]
#[post("/queue/remove/{index}")]
/// Remove the music in queue at index
async fn remove() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}
