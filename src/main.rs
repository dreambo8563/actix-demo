use actix_web::{App, HttpServer};
use demo::ENV;
use demo::LOGGING;
use slog::{info, warn};

//FIXME: rm env_logger
// slog is the log package

//TODO: find a way to supper log rotate

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    demo::init();

    let url = &ENV.database.url;
    let applogger = &LOGGING.logger;
    info!(applogger, "env"; "ENV.database"=> format!("{:?}", ENV.database));
    warn!(applogger,"111"; "url" => url);
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
