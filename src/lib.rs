#[macro_use]
extern crate log;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Route;

mod controllers;
mod errors;
mod routes;
mod utils;

pub fn init_log() {
    env_logger::init();
}

pub fn query_config() -> actix_web::web::QueryConfig {
    // change query extractor configuration
    web::QueryConfig::default().error_handler(|err, _| {
        // <- create custom error response
        // 定制 QueryConfig 的错误
        errors::MyError::BadRequestData(err).into()
    })
}

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    routes::user::routes(cfg);
}

pub fn default_route() -> Route {
    web::to(|| HttpResponse::NotFound())
}
