use secrecy::Secret;

pub struct AuthTokens {
    access_token: Secret<String>,
}

pub fn check_credentials(_password: Secret<String>) -> bool {
    true
}

pub fn generate_jwt() -> AuthTokens {
    AuthTokens {
        access_token: Secret::new("todo".into()),
    }
}
