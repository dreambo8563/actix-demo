use actix_web::{App, HttpServer};
use demo::Settings;
use lazy_static::lazy_static;
use log::info;

lazy_static! {
    static ref ENV: Settings = Settings::new().unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    demo::init();
    // let s: demo::settings::Settings = demo::settings::Settings::new().unwrap();
    info!("{:?}", ENV.database);
    // info!("start");
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
