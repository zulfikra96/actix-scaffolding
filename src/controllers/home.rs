use actix_web::HttpResponse;
use actix_web::get;
#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world")
}