use actix_web::HttpResponse;
use actix_web::get;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct HomeView {}

#[get("/")]
async fn index() -> HttpResponse {
    let view = HomeView {};
    HttpResponse::Ok().body(view.render_once().unwrap())
}

