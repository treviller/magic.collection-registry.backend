use actix_web::body::BoxBody;
use actix_web::{HttpResponse, HttpResponseBuilder};

pub mod domain;
pub mod jwt;

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;

    let mut current = e.source();

    while let Some(cause) = current {
        writeln!(f, "Caused by : \n\t{}", cause)?;
        current = cause.source();
    }

    Ok(())
}

#[derive(serde::Serialize)]
pub struct ErrorResponseDto {
    meta: ErrorResponseMeta,
    data: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ErrorResponseMeta {
    error_code: String,
    error_message: String,
}

pub trait FormattedResponseError {
    fn error_code(&self) -> String;
    fn error_message(&self) -> String;

    fn format_response(
        &self,
        response: &mut HttpResponseBuilder,
        error_code: String,
        error_message: String,
    ) -> HttpResponse<BoxBody> {
        response.json(ErrorResponseDto {
            meta: ErrorResponseMeta {
                error_code,
                error_message,
            },
            data: None,
        })
    }
}
