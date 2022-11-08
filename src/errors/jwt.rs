use std::fmt::Formatter;

use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, ResponseError};

use crate::errors::{error_chain_fmt, FormattedResponseError};

#[derive(thiserror::Error)]
pub enum JwtError {
    #[error("Invalid token.")]
    InvalidToken(#[from] anyhow::Error),
    #[error("Token expired.")]
    TokenExpired(#[source] anyhow::Error),
}

impl std::fmt::Debug for JwtError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl FormattedResponseError for JwtError {
    fn error_code(&self) -> String {
        match self {
            JwtError::InvalidToken(_) => "invalid_token".into(),
            JwtError::TokenExpired(_) => "token_expired".into(),
        }
    }

    fn error_message(&self) -> String {
        self.to_string()
    }
}

impl ResponseError for JwtError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut response = HttpResponse::Unauthorized();

        response.insert_header(ContentType::json());

        self.format_response(&mut response, self.error_code(), self.error_message())
    }
}
