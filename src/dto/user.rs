use crate::domain::model::user::User;

#[derive(serde::Serialize)]
pub struct UserDto {
    username: String,
}

impl UserDto {
    pub fn from_user(user: User) -> Self {
        Self {
            username: user.username,
        }
    }
}
