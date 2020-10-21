#[macro_use]
extern crate log;

use actix_web::{error, web, App, HttpResponse, HttpServer};

mod controllers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("start");
    HttpServer::new(|| {
        App::new()
            .app_data(
                // change query extractor configuration
                web::QueryConfig::default().error_handler(|err, _| {
                    // <- create custom error response
                    // 定制 QueryConfig 的错误
                    error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                        .into()
                }),
            )
            .configure(routes::user::routes)
            // default route
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
