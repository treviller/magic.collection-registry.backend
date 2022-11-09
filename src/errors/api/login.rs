use std::fmt::Formatter;

use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use crate::errors::auth::AuthError;
use crate::errors::{error_chain_fmt, FormattedResponseError};

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl FormattedResponseError for LoginError {
    fn error_code(&self) -> String {
        match self {
            LoginError::InvalidCredentials(_) => "invalid_credentials".into(),
            _ => "internal_error".into(),
        }
    }

    fn error_message(&self) -> String {
        self.to_string()
    }
}

impl ResponseError for LoginError {
    fn status_code(&self) -> StatusCode {
        match self {
            LoginError::InvalidCredentials(_) => StatusCode::UNAUTHORIZED,
            LoginError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut response = HttpResponse::build(self.status_code());

        response.insert_header(ContentType::json());

        self.format_response(&mut response, self.error_code(), self.error_message())
    }
}

impl From<AuthError> for LoginError {
    fn from(error: AuthError) -> Self {
        match error {
            AuthError::InvalidCredentials(e) => LoginError::InvalidCredentials(e),
            AuthError::UnexpectedError(e) => LoginError::UnexpectedError(e),
        }
    }
}
