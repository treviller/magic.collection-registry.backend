use diesel::debug_query;
use diesel::pg::Pg;
use diesel::prelude::*;
use secrecy::{ExposeSecret, Secret};
use uuid::Uuid;

use crate::domain::model::user::User;
use crate::provider::database::establish_connection;
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

pub struct DbUserProvider {}

impl UserProvider for DbUserProvider {
    fn find_one_by_username(&self, searched_username: &str) -> Option<User> {
        let mut connection = establish_connection();

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
        let mut connection = establish_connection();

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
        let mut connection = establish_connection();
        let user: DbUser = user.into();

        tracing::info!("USER DEBUG {:?}", user);

        //TODO handle all error cases
        let result = diesel::update(&user).set(&user);

        let sql = debug_query::<Pg, _>(&result);
        println!("DEBUG QUERY {:?}", sql);

        let result = result.execute(&mut connection);

        match result {
            Ok(value) => tracing::info!("OK : {}", value),
            Err(value) => tracing::error!("ERROR : {}", value),
        };
    }
}
