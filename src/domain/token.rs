use secrecy::Secret;
use uuid::Uuid;

use crate::authentication::AuthenticationService;
use crate::configuration::settings::Settings;
use crate::domain::model::token::{Token, TokenType};
use crate::domain::model::user::User;
use crate::domain::user::UserService;
use crate::errors::domain::DomainError;
use crate::provider::database::token::DbTokenProvider;
use crate::provider::database::DbConnection;
use crate::provider::token::TokenProvider;

pub struct TokenService<'a> {
    token_provider: DbTokenProvider<'a>,
    user_service: UserService<'a>,
    auth_service: AuthenticationService,
}

impl<'a> TokenService<'a> {
    pub fn new(config: &Settings, db_pool: &'a DbConnection) -> Self {
        let user_service = UserService::new(db_pool);
        let auth_service = AuthenticationService::new(config.auth.clone());

        Self {
            token_provider: DbTokenProvider::new(db_pool),
            user_service,
            auth_service,
        }
    }

    pub fn reset_user_password(
        &mut self,
        token_id: Uuid,
        password: &Secret<String>,
    ) -> Result<(), DomainError> {
        let token = self.token_provider.find_token_by_id(token_id).unwrap();
        let user = self.user_service.get_user_from_id(token.user_id)?;

        match token.token_type {
            TokenType::ResetPassword => {
                let password_hash = self.auth_service.hash_password(&password);

                self.user_service.update_user_password(&user, password_hash);
            }
        }

        Ok(())
    }

    pub fn generate_token_for_user(&self, user: &User) -> Result<Token, DomainError> {
        let token = Token {
            id: Uuid::new_v4(),
            token_type: TokenType::ResetPassword,
            user_id: user.id,
        };

        self.token_provider.save_token(token.clone());

        Ok(token)
    }
}
