use actix_web::dev::HttpResponseBuilder;
use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use error::ResponseError;
use serde::Serialize;

#[derive(Serialize)]
struct CommonErr<'a> {
    err: &'a str,
}

#[derive(Debug, Display, Error)]
pub enum MyError {
    BadRequestData(error::QueryPayloadError),
}

impl ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::BadRequestData(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match &self {
            MyError::BadRequestData(e) => {
                HttpResponseBuilder::new(self.status_code()).json(CommonErr {
                    err: format!("{}", e).as_str(),
                })
            }
        }
    }
}
