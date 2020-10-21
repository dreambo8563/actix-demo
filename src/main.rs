#[macro_use]
extern crate log;
use log::info;

use actix_web::{web, App, HttpResponse, HttpServer};

mod controllers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("start");
    HttpServer::new(|| {
        App::new()
            .configure(routes::user::routes)
            // default route
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
