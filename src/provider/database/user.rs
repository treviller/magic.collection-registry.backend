use secrecy::{ExposeSecret, Secret};
use uuid::Uuid;

use crate::domain::model::user::User;
use crate::provider::database::DbConnection;

#[derive(sqlx::FromRow)]
pub struct DbUser {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

impl Into<User> for DbUser {
    fn into(self) -> User {
        User {
            id: self.id,
            username: self.username,
            password: Secret::new(self.password),
        }
    }
}

impl From<User> for DbUser {
    fn from(user: User) -> Self {
        DbUser {
            id: user.id,
            username: user.username,
            password: user.password.expose_secret().into(),
        }
    }
}

pub async fn find_one_by_username(db_pool: &DbConnection, searched_username: &str) -> Option<User> {
    let result: Option<DbUser> = sqlx::query_as!(
        DbUser,
        "SELECT * FROM users WHERE username = $1 LIMIT 1",
        searched_username
    )
    .fetch_optional(db_pool)
    .await
    .expect("No error okay");

    match result {
        Some(user) => Some(user.into()),
        None => None, //TODO handle all error cases
    }
}

pub async fn find_one_by_id(db_pool: &DbConnection, user_id: Uuid) -> Option<User> {
    let result: Option<DbUser> =
        sqlx::query_as!(DbUser, "SELECT * FROM users WHERE id = $1 LIMIT 1", user_id)
            .fetch_optional(db_pool)
            .await
            .expect("No error okay");

    match result {
        Some(user) => Some(user.into()),
        None => None, //TODO handle all error cases
    }
}

pub async fn update_user_password(
    db_pool: &DbConnection,
    user_id: Uuid,
    new_password: Secret<String>,
) {
    sqlx::query!(
        "UPDATE users SET password = $1 WHERE id = $2",
        new_password.expose_secret(),
        user_id
    )
    .execute(db_pool)
    .await
    .expect("No error okay"); //TODO handle all error cases
}
