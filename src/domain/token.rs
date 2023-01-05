use secrecy::Secret;
use uuid::Uuid;

use crate::authentication::AuthenticationService;
use crate::configuration::settings::Settings;
use crate::domain::model::token::{Token, TokenType};
use crate::domain::model::user::User;
use crate::domain::user::UserService;
use crate::errors::domain::DomainError;
use crate::provider::database;
use crate::provider::database::DbConnection;

pub struct TokenService<'a> {
    db_pool: &'a DbConnection,
    user_service: UserService<'a>,
    auth_service: AuthenticationService,
}

impl<'a> TokenService<'a> {
    pub fn new(config: &Settings, db_pool: &'a DbConnection) -> Self {
        let user_service = UserService::new(db_pool);
        let auth_service = AuthenticationService::new(config.auth.clone());

        Self {
            db_pool,
            user_service,
            auth_service,
        }
    }

    pub async fn reset_user_password(
        &mut self,
        token_id: Uuid,
        password: &Secret<String>,
    ) -> Result<(), DomainError> {
        let token = database::token::find_token_by_id(self.db_pool, token_id)
            .await
            .unwrap();
        let user = self.user_service.get_user_from_id(token.user_id).await?;

        match token.token_type {
            TokenType::ResetPassword => {
                let password_hash = self.auth_service.hash_password(&password);

                self.user_service
                    .update_user_password(&user, password_hash)
                    .await;
            }
        }

        Ok(())
    }

    pub async fn generate_token_for_user(&self, user: &User) -> Result<Token, DomainError> {
        let token = Token {
            id: Uuid::new_v4(),
            token_type: TokenType::ResetPassword,
            user_id: user.id,
        };

        database::token::save_token(self.db_pool, token.clone()).await;

        Ok(token)
    }
}
