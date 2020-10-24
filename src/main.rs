use actix_web::{App, HttpServer};
use demo::ENV;
use demo::LOGGING;
use log::info;
use slog::warn;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    demo::init();
    info!("{:?}", ENV.database);
    let url = &ENV.database.url;
    let applogger = &LOGGING.logger;
    warn!(applogger, "Service starting"; "url" => url);
    HttpServer::new(|| {
        App::new()
            .app_data(demo::ExtractorConifg::query_config())
            .configure(demo::Routes::config)
            // default route
            .default_service(demo::Routes::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
