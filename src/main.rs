mod controllers;
mod routes;

use actix_cors::Cors;
use actix_web::{http, App, HttpServer};
use dotenv::dotenv;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors) 
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}