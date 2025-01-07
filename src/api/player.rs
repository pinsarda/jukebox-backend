use actix_web::{ HttpResponse, Responder};
use paperclip::actix::{ api_v2_operation, get, web::{ self, Json }};

#[api_v2_operation]
#[get("/player/play")]
/// Start playback of enqueued music
async fn play() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[api_v2_operation]
#[get("/player/stop")]
/// Stop music playback
async fn stop() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[api_v2_operation]
#[get("/player/next")]
/// Skip to next music in queue
async fn next() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[api_v2_operation]
#[get("/player/previous")]
/// Skip to previous music in queue
async fn previous() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}
