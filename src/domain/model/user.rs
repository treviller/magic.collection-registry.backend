use secrecy::Secret;

#[derive(Clone)]
pub struct User {
    pub username: String,
    pub password: Secret<String>,
}
