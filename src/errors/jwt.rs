use std::fmt::Formatter;

use actix_web::ResponseError;

#[derive(thiserror::Error)]
pub enum JwtError {
    #[error("Invalid token.")]
    InvalidToken(#[from] anyhow::Error),
    #[error("Token expired.")]
    TokenExpired(#[source] anyhow::Error),
}

impl std::fmt::Debug for JwtError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\n", self)?;

        Ok(())
    }
}

impl ResponseError for JwtError {}
