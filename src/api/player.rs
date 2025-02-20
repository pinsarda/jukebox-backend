use actix_web::{ get, post, HttpResponse, Responder };

#[post("/player/play")]
/// Start playback of enqueued music
async fn play() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[post("/player/stop")]
/// Stop music playback
async fn stop() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[post("/player/next")]
/// Skip to next music in queue
async fn next() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[post("/player/previous")]
/// Skip to previous music in queue
async fn previous() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[post("/player/seek")]
/// Seek to 
async fn seek() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[get("/player/state")]
/// Get player state
async fn state() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

// TODO : add /player/socket for live update on all clients