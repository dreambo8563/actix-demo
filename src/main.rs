use actix_web::{get, guard, web, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;

#[get("/index")]
async fn index() -> impl Responder {
    "Hello world!"
}

#[get("/echo")]
async fn index2() -> impl Responder {
    "Hello world2!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .service(
                // prefixes all resources and routes attached to it...
                web::scope("/api/v1")
                    // ...so this handles requests for `GET /app/index.html`
                    .service(index),
            )
            .service(
                web::scope("/api/v2")
                    // guard for this service
                    .guard(guard::Header("xx", "users.rust-lang.org"))
                    .service(index2),
            )
            // default route
            .default_service(web::to(|| HttpResponse::NoContent()))
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}
