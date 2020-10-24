mod controllers;
mod errors;
mod routes;
mod settings;
mod utils;

use actix_web::{web, HttpResponse, Route};

pub use settings::Settings;

pub fn init() {
    env_logger::init();
}

pub struct Routes;

impl Routes {
    pub fn default() -> Route {
        default_route()
    }
    pub fn config(cfg: &mut web::ServiceConfig) {
        routes_config(cfg);
    }
}

pub struct ExtractorConifg;

impl ExtractorConifg {
    pub fn query_config() -> actix_web::web::QueryConfig {
        // change query extractor configuration
        web::QueryConfig::default().error_handler(|err, _| {
            // <- create custom error response
            // 定制 QueryConfig 的错误
            errors::MyError::BadRequestData(err).into()
        })
    }
}

fn routes_config(cfg: &mut web::ServiceConfig) {
    routes::user::routes(cfg);
}

fn default_route() -> Route {
    web::to(|| HttpResponse::NotFound())
}
