use uuid::Uuid;

use crate::domain::model::user::User;

pub trait UserProvider {
    fn find_one_by_username(&self, username: &str) -> Option<User>;
    fn find_one_by_id(&self, user_id: Uuid) -> Option<User>;
    fn update_user(&mut self, user: User);
}
