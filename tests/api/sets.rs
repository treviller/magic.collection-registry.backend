use actix_web::http::header::ContentType;
use actix_web::test;

use magic_collection_registry_backend::configuration::loader::get_configuration;
use magic_collection_registry_backend::provider::database::DbConnection;
use magic_collection_registry_backend::routes::responses::sets::SetListResponse;

use crate::helpers;

#[sqlx::test(fixtures("sets"))]
pub async fn sets_list_should_return_200(db_pool: DbConnection) {
    let configuration = get_configuration().expect("Failed to build configuration.");

    let req = test::TestRequest::get()
        .uri("/api/sets")
        .insert_header(ContentType::json());

    let response = helpers::init_test_app_and_make_request(db_pool, configuration, req).await;
    assert!(response.status().is_success());

    let _json: SetListResponse = test::read_body_json(response).await;
}
