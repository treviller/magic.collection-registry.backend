use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use secrecy::{ExposeSecret, Secret};
use uuid::Uuid;

use crate::domain::model::user::User;
use crate::provider::user::UserProvider;
use crate::schema::users;
use crate::schema::users::dsl::*;

#[derive(AsChangeset, Queryable, Identifiable, Insertable, Debug)]
#[diesel(table_name = users)]
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

pub struct DbUserProvider<'a> {
    db_pool: &'a Pool<ConnectionManager<PgConnection>>,
}

impl<'a> DbUserProvider<'a> {
    pub fn new(db_pool: &'a Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { db_pool }
    }
}

impl<'a> UserProvider for DbUserProvider<'a> {
    fn find_one_by_username(&self, searched_username: &str) -> Option<User> {
        let mut connection = self.db_pool.get().unwrap();

        let result: QueryResult<DbUser> = users
            .filter(username.eq(searched_username))
            .limit(1)
            .get_result::<DbUser>(&mut connection);

        match result {
            Ok(user) => Some(user.into()),
            Err(_error) => None, //TODO handle all error cases
        }
    }

    fn find_one_by_id(&self, user_id: Uuid) -> Option<User> {
        let mut connection = self.db_pool.get().unwrap();

        let result: QueryResult<DbUser> = users
            .filter(id.eq(user_id))
            .limit(1)
            .get_result::<DbUser>(&mut connection);

        match result {
            Ok(user) => Some(user.into()),
            Err(_error) => None, //TODO handle all error cases
        }
    }

    fn update_user(&mut self, user: User) {
        let mut connection = self.db_pool.get().unwrap();
        let user: DbUser = user.into();

        //TODO handle all error cases
        let _ = diesel::update(&user).set(&user).execute(&mut connection);
    }
}
