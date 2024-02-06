mod config;
mod controllers;
mod routes;

use crate::config::database::establish_connection;
use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use controllers::home;
use dotenv::dotenv;
use actix_files as fs;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;


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
            .service(fs::Files::new("/public", "public").show_files_listing())
            .wrap(cors)
            .app_data(web::Data::new(establish_connection().clone()))
            .service(home::index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
