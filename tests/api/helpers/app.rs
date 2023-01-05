use actix_web::dev::ServiceResponse;
use actix_web::{test, web, App};
use once_cell::sync::Lazy;
use secrecy::Secret;
use tracing_actix_web::TracingLogger;
use wiremock::MockServer;

use magic_collection_registry_backend::app::{configure_routing, initialize_tera};
use magic_collection_registry_backend::authentication::AuthenticationService;
use magic_collection_registry_backend::configuration::settings::Settings;
use magic_collection_registry_backend::monitoring::{get_subscriber, initialize_subscriber};
use magic_collection_registry_backend::provider::database;
use magic_collection_registry_backend::provider::database::DbConnection;

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("info".into());
    initialize_subscriber(subscriber);
});

pub fn test_setup() {
    Lazy::force(&TRACING);
}

pub async fn init_test_app_and_make_request(
    db_pool: DbConnection,
    configuration: Settings,
    request: test::TestRequest,
) -> ServiceResponse {
    test_setup();

    let config_data = web::Data::new(configuration);
    let tera = web::Data::new(initialize_tera());
    let db_pool = web::Data::new(db_pool);

    let app = App::new()
        .wrap(TracingLogger::default())
        .app_data(config_data.clone())
        .app_data(tera.clone())
        .app_data(db_pool.clone())
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

pub async fn generate_access_token(db_pool: &DbConnection, config: &Settings) -> Secret<String> {
    let authentication_service = AuthenticationService::new(config.auth.clone());
    let token = authentication_service
        .generate_jwt(
            database::user::find_one_by_username(db_pool, "test@email.com".into())
                .await
                .unwrap(),
        )
        .unwrap();

    token.access_token
}
