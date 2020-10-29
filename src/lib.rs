#![crate_name = "demo"]
#[macro_use]
extern crate slog;
#[macro_use]
extern crate actix_web;
use actix_files::Files;
use actix_web::{App, HttpServer};
// #[macro_use]
// extern crate slog;
// global import macro in the crate

mod controllers;
mod errors;
mod routes;
mod utils;
// use utils::logger;
// use utils::settings;

use actix_web::{web, HttpResponse, Route};

pub use utils::ENV;
pub use utils::LOGGING as logger;

/// preparation for this app
pub fn init() {
    env_logger::init();
}
/// Route info
struct Routes;

impl Routes {
    /// return default route of the app
    fn default() -> Route {
        default_route()
    }
    /// return config for all the app routes
    fn config(cfg: &mut web::ServiceConfig) {
        debug!(&logger, "route config start");
        routes_config(cfg);
    }
}

/// base struct of all the configs of Extractor
struct ExtractorConifg;

impl ExtractorConifg {
    /// customized config for query
    fn query_config() -> actix_web::web::QueryConfig {
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
#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(ExtractorConifg::query_config())
            .configure(Routes::config)
            // default route
            .default_service(Routes::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
