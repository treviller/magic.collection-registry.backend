use std::sync::Once;

use actix_web::dev::ServiceResponse;
use actix_web::{test, web, App};
use dotenvy::dotenv;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use secrecy::Secret;
use tracing_actix_web::TracingLogger;
use wiremock::MockServer;

use magic_collection_registry_backend::app::{configure_routing, initialize_tera};
use magic_collection_registry_backend::authentication::AuthenticationService;
use magic_collection_registry_backend::configuration::settings::Settings;
use magic_collection_registry_backend::monitoring::{get_subscriber, initialize_subscriber};
use magic_collection_registry_backend::provider::database::user::DbUserProvider;
use magic_collection_registry_backend::provider::database::DbConnection;
use magic_collection_registry_backend::provider::user::UserProvider;

use crate::helpers::database::establish_test_connection_pool;
use crate::helpers::fixtures::load_fixtures;

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("info".into());
    initialize_subscriber(subscriber);
});

static INIT_ENVIRONMENT: Once = Once::new();

lazy_static! {
    static ref DB_POOL: DbConnection = establish_test_connection_pool();
}

pub fn test_setup() {
    Lazy::force(&TRACING);

    INIT_ENVIRONMENT.call_once(|| {
        load_env_values();
        load_fixtures();
    });
}

pub async fn init_test_app_and_make_request(
    configuration: Settings,
    request: test::TestRequest,
) -> ServiceResponse {
    test_setup();

    let config_data = web::Data::new(configuration);
    let tera = web::Data::new(initialize_tera());
    let db_pool = web::Data::new(DB_POOL.clone());

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

pub fn generate_access_token(config: &Settings) -> Secret<String> {
    let authentication_service = AuthenticationService::new(config.auth.clone());
    let user_provider = DbUserProvider::new(&DB_POOL);
    let token = authentication_service
        .generate_jwt(
            user_provider
                .find_one_by_username("test@email.com".into())
                .unwrap(),
        )
        .unwrap();

    token.access_token
}

fn load_env_values() {
    dotenv().ok();

    let iter = dotenvy::from_filename_iter(".env.test").unwrap();
    iter.overload().unwrap();
}
