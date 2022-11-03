use actix_web::web;

use crate::configuration::settings::Settings;
use crate::domain::authentication::AuthenticationService;
use crate::provider::memory::user::UserMemoryProvider;

#[derive(Clone)]
pub struct ServiceContainer {
    pub authentication_service: web::Data<AuthenticationService>,
}

impl ServiceContainer {
    pub fn new(configuration: Settings) -> Self {
        let user_provider = UserMemoryProvider::new();

        let authentication_service = web::Data::new(AuthenticationService::new(
            user_provider,
            configuration.jwt_key,
            configuration.jwt_ttl,
        ));

        Self {
            authentication_service,
        }
    }
}
