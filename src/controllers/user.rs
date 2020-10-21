use actix_web::{get, Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;

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
