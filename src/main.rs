extern crate log;

use actix_web::{web, App, HttpResponse, HttpServer};
use listenfd::ListenFd;

mod controllers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .configure(routes::user::routes)
            // default route
            .default_service(web::to(|| HttpResponse::NotFound()))
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}
