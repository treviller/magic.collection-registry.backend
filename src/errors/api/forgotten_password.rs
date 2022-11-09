use std::fmt::Formatter;

use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use crate::errors::{error_chain_fmt, FormattedResponseError};

#[derive(thiserror::Error)]
pub enum ForgottenPasswordError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("{0}")]
    ValidationError(String),
}

impl std::fmt::Debug for ForgottenPasswordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl FormattedResponseError for ForgottenPasswordError {
    fn error_code(&self) -> String {
        match self {
            ForgottenPasswordError::ValidationError(_) => "invalid_email".into(),
            ForgottenPasswordError::UnexpectedError(_) => "internal_error".into(),
        }
    }

    fn error_message(&self) -> String {
        self.to_string()
    }
}

impl ResponseError for ForgottenPasswordError {
    fn status_code(&self) -> StatusCode {
        match self {
            ForgottenPasswordError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ForgottenPasswordError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut response = HttpResponse::build(self.status_code());

        response.insert_header(ContentType::json());

        self.format_response(&mut response, self.error_code(), self.error_message())
    }
}
