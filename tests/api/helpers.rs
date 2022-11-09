use actix_web::dev::ServiceResponse;
use actix_web::{test, web, App};
use once_cell::sync::Lazy;
use secrecy::Secret;
use tracing_actix_web::TracingLogger;
use wiremock::MockServer;

use magic_collection_registry_backend::app::configure_routing;
use magic_collection_registry_backend::authentication::AuthenticationService;
use magic_collection_registry_backend::configuration::settings::Settings;
use magic_collection_registry_backend::monitoring::{get_subscriber, initialize_subscriber};
use magic_collection_registry_backend::provider::memory::user::UserMemoryProvider;
use magic_collection_registry_backend::provider::user::UserProvider;

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("info".into());
    initialize_subscriber(subscriber);
});

pub async fn init_test_app_and_make_request(
    configuration: Settings,
    request: test::TestRequest,
) -> ServiceResponse {
    Lazy::force(&TRACING);
    let config_data = web::Data::new(configuration);

    let app = App::new()
        .wrap(TracingLogger::default())
        .app_data(config_data.clone())
        .configure(configure_routing);

    let test_app = test::init_service(app).await;

    test::call_service(&test_app, request.to_request())
        .await
        .map_into_boxed_body()
}

pub async fn mock_email_server(config: &mut Settings) -> MockServer {
    let email_server = MockServer::start().await;

    config.email.base_url = email_server.uri();

    email_server
}

pub fn generate_access_token(config: &Settings) -> Secret<String> {
    let authentication_service = AuthenticationService::new(config.auth.clone());
    let user_provider = UserMemoryProvider::new();
    let token = authentication_service
        .generate_jwt(user_provider.find_one_by_username("user1".into()).unwrap())
        .unwrap();

    token.access_token
}
