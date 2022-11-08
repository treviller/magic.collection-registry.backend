use std::fmt::Formatter;

use actix_web::ResponseError;

use crate::domain::model::error_chain_fmt;

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

impl ResponseError for JwtError {}
