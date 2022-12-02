use crate::errors::{error_chain_fmt, FormattedResponseError};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use std::fmt::Formatter;

#[derive(thiserror::Error)]
pub enum CardListError {
    #[error("{0}")]
    ValidationError(String),
}

impl std::fmt::Debug for CardListError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl FormattedResponseError for CardListError {
    fn error_code(&self) -> String {
        "invalid_data".into()
    }

    fn error_message(&self) -> String {
        self.to_string()
    }
}

impl ResponseError for CardListError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut response = HttpResponse::build(self.status_code());

        response.insert_header(ContentType::json());

        self.format_response(&mut response, self.error_code(), self.error_message())
    }
}
