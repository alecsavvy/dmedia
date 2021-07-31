use std::result::Result as GenericResult;

use actix_web::ResponseError;
use strum_macros::Display;

pub type Result<T> = GenericResult<T, AppError>;

#[derive(Debug, Display)]
pub enum AppError {}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let res = actix_web::HttpResponse::new(self.status_code());
        res
    }
}
