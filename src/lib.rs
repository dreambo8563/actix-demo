#[macro_use]
extern crate slog;
#[macro_use]
extern crate actix_web;

// #[macro_use]
// extern crate slog;
// global import macro in the crate

mod controllers;
mod errors;
mod routes;
mod utils;
// use utils::logger;
use utils::settings;

use actix_web::{web, HttpResponse, Route};

pub use settings::Settings;
pub use utils::ENV;
pub use utils::LOGGING;

pub fn init() {
    env_logger::init();
}

pub struct Routes;

impl Routes {
    pub fn default() -> Route {
        default_route()
    }
    pub fn config(cfg: &mut web::ServiceConfig) {
        debug!(&LOGGING.logger, "route config start");
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
