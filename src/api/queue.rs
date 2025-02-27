use actix_web::{ HttpResponse, Responder, get, post };


#[utoipa::path()]
#[get("/queue/state")]
/// Get state of the queue
async fn state() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[utoipa::path()]
#[post("/queue/add/{music_id}")]
/// Enqueue a music
async fn add() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}

#[utoipa::path()]
#[post("/queue/remove/{index}")]
/// Remove the music in queue at index
async fn remove() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}
