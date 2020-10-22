#[macro_use]
extern crate log;
use actix_web::{error, web, HttpResponse};
use serde::Serialize;
mod controllers;
mod routes;
mod utils;

#[derive(Serialize)]
struct MyObj<'a> {
    message: &'a str,
}

pub fn init_log() {
    env_logger::init();
}

pub fn extractors_config() -> actix_web::web::QueryConfig {
    // change query extractor configuration
    web::QueryConfig::default().error_handler(|err, _| {
        // <- create custom error response
        // 定制 QueryConfig 的错误
        debug!("{:?}", &err);
        // let msg = err.to_string();
        error::InternalError::from_response(
            "error",
            HttpResponse::Ok().json(MyObj {
                message: format!("{}", err).as_str(),
            }),
        )
        .into()
    })
}

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    routes::user::routes(cfg);
}
