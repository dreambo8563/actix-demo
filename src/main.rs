#[macro_use]
extern crate log;

use actix_web::{error, web, App, HttpResponse, HttpServer};
use serde::Serialize;
mod controllers;
mod routes;

#[derive(Serialize)]
struct MyObj {
    message: String,
}

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
                    debug!("{:?}", &err);
                    let msg = err.to_string();
                    error::InternalError::from_response(
                        err,
                        HttpResponse::Ok().json(MyObj {
                            message: format!("{}", msg),
                        }),
                    )
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
