use std::fmt::Formatter;

use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use crate::errors::{error_chain_fmt, FormattedResponseError};

#[derive(thiserror::Error)]
pub enum ResetPasswordError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("The token is invalid.")]
    InvalidToken(#[source] anyhow::Error),
}

impl std::fmt::Debug for ResetPasswordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl FormattedResponseError for ResetPasswordError {
    fn error_code(&self) -> String {
        match self {
            ResetPasswordError::UnexpectedError(_) => "internal_error".into(),
            ResetPasswordError::InvalidToken(_) => "invalid_token".into(),
        }
    }

    fn error_message(&self) -> String {
        self.to_string()
    }
}

impl ResponseError for ResetPasswordError {
    fn status_code(&self) -> StatusCode {
        match self {
            ResetPasswordError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ResetPasswordError::InvalidToken(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut response = HttpResponse::build(self.status_code());

        response.insert_header(ContentType::json());

        self.format_response(&mut response, self.error_code(), self.error_message())
    }
}
