use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder, Result};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
    age: u32,
}

// Responder
impl Responder for MyObj {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}
#[get("")]
pub async fn index() -> impl Responder {
    info!("enter the user index");
    MyObj {
        name: "user",
        age: 88,
    }
}

#[derive(Deserialize)]
pub struct Info {
    user_id: u32,
    friend: String,
}

#[derive(Deserialize)]
pub struct Name {
    name: String,
}

#[get("/{user_id}/{friend}")]
pub async fn friend(info: web::Path<Info>, query: web::Query<Name>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}, name: {}!",
        info.friend, info.user_id, query.name
    ))
}
