use actix_web::{web, App, HttpResponse, HttpServer};
use listenfd::ListenFd;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .configure(routes::user::user_routes)
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
