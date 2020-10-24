use crate::utils::wrapper::res_success;
use actix_web::{get, web, Responder, Result};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
    age: u32,
}

#[get("")]
pub async fn index() -> impl Responder {
    info!("enter the user index");

    res_success(MyObj {
        name: "user",
        age: 88,
    })
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
