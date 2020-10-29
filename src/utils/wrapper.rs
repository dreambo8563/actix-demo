// use actix_web::web::Json;
use actix_web::{HttpResponse, Result};
use serde::Serialize;

#[derive(Serialize)]
struct SuccessResponse<T: Serialize> {
    data: T,
}

/// response wrapper to make reponse struct consistent
pub fn res_success<T>(res: T) -> Result<HttpResponse>
where
    T: Serialize,
{
    Ok(HttpResponse::Ok().json(SuccessResponse { data: res }))
}
