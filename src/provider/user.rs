use crate::domain::model::user::User;

pub trait UserProvider {
    fn find_one_by_username(&self, username: &str) -> Option<User>;
}
