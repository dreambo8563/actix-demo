use actix_web::{get, guard, web, Responder};

#[get("/user")]
async fn name() -> impl Responder {
    "user name!"
}

// this function could be located in a different module
fn user_routes_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1").service(name));
}

// this function could be located in a different module
fn user_routes_v2(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v2")
            .guard(guard::Header("xx", "users.rust-lang.org"))
            .service(name),
    );
}

// this function could be located in a different module
pub fn user_routes(cfg: &mut web::ServiceConfig) {
    user_routes_v1(cfg);
    user_routes_v2(cfg);
}
