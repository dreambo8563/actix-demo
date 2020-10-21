use actix_web::{get, guard, web, App, HttpResponse, HttpServer, Responder};

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
    HttpServer::new(|| {
        App::new()
            .service(
                // prefixes all resources and routes attached to it...
                web::scope("/api/v1")
                    // ...so this handles requests for `GET /app/index.html`
                    .service(index),
            )
            .service(
                web::scope("/api/v2")
                    .guard(guard::Header("xx", "users.rust-lang.org"))
                    .service(index2),
            )
            // default route
            .default_service(web::to(|| HttpResponse::NoContent()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
