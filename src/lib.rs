#![crate_name = "demo"]
#[macro_use]
extern crate slog;
#[macro_use]
extern crate actix_web;
use actix_files::Files;
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
pub use utils::LOGGING as logger;

/// preparation for this app
pub fn init() {
    env_logger::init();
}
/// Route info
pub struct Routes;

impl Routes {
    /// return default route of the app
    pub fn default() -> Route {
        default_route()
    }
    /// return config for all the app routes
    pub fn config(cfg: &mut web::ServiceConfig) {
        debug!(&logger, "route config start");
        routes_config(cfg);
    }
}

/// base struct of all the configs of Extractor
pub struct ExtractorConifg;

impl ExtractorConifg {
    /// customized config for query
    pub fn query_config() -> actix_web::web::QueryConfig {
        // change query extractor configuration
        web::QueryConfig::default().error_handler(|err, _| {
            // <- create custom error response
            // 定制 QueryConfig 的错误
            errors::MyError::BadRequestData(err).into()
        })
    }
}

/// compose route configs from all modules
fn routes_config(cfg: &mut web::ServiceConfig) {
    routes::user::routes(cfg);
    static_route(cfg);
}

/// default route lead to 404 page
fn default_route() -> Route {
    web::to(|| HttpResponse::NotFound())
}

/// default route lead to 404 page
fn static_route(cfg: &mut web::ServiceConfig) {
    cfg.service(Files::new("/", "public/").show_files_listing());
}
