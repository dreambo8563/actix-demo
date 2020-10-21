use crate::controllers;
use actix_web::{get, guard, web, Responder};

#[get("/user")]
async fn name() -> impl Responder {
    "user name!"
}

// this function could be located in a different module
fn routes_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1/user").service(name));
}

// this function could be located in a different module
fn routes_v2(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v2/user")
            .guard(guard::Header("xx", "users.rust-lang.org"))
            .service(controllers::user::index)
            .service(controllers::user::friend),
    );
}

// this function could be located in a different module
pub fn routes(cfg: &mut web::ServiceConfig) {
    routes_v1(cfg);
    routes_v2(cfg);
}
