use std::fmt::Formatter;

use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, ResponseError};

use crate::errors::{error_chain_fmt, FormattedResponseError};

#[derive(thiserror::Error)]
pub enum ForgottenPasswordError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for ForgottenPasswordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl FormattedResponseError for ForgottenPasswordError {
    fn error_code(&self) -> String {
        "internal_error".into()
    }

    fn error_message(&self) -> String {
        self.to_string()
    }
}

impl ResponseError for ForgottenPasswordError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut response = HttpResponse::InternalServerError();

        response.insert_header(ContentType::json());

        self.format_response(&mut response, self.error_code(), self.error_message())
    }
}
