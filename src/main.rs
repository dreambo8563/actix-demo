use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    demo::init_log();
    // info!("start");
    HttpServer::new(|| {
        App::new()
            .app_data(demo::extractors_config())
            .configure(demo::routes_config)
            // default route
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
