use urlencoding::encode;

pub use app::*;

mod app;
pub mod database;

pub fn add_query_parameters(uri: &str, parameters: &mut Vec<(&str, &str)>) -> String {
    let first_param = match parameters.pop() {
        Some(parameter) => parameter,
        None => return uri.into(),
    };

    let mut uri = format!(
        "{}?{}={}",
        uri,
        encode(first_param.0),
        encode(first_param.1)
    );

    for parameter in parameters {
        uri = format!("{}&{}={}", uri, encode(parameter.0), encode(parameter.1));
    }

    uri
}
