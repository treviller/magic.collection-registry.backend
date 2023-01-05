use actix_web::http::header::ContentType;
use actix_web::test;

use magic_collection_registry_backend::configuration::loader::get_configuration;
use magic_collection_registry_backend::domain::model::card::CardRarity;
use magic_collection_registry_backend::provider::database::DbConnection;
use magic_collection_registry_backend::routes::responses::cards::CardsListResponse;

use crate::helpers;
use crate::helpers::add_query_parameters;

#[sqlx::test(fixtures("sets", "cards"))]
pub async fn cards_list_should_return_200(db_pool: DbConnection) {
    let configuration = get_configuration().expect("Failed to build configuration.");

    let req = test::TestRequest::get()
        .uri("/api/cards")
        .insert_header(ContentType::json());

    let response = helpers::init_test_app_and_make_request(db_pool, configuration, req).await;
    assert!(response.status().is_success());

    let _json: CardsListResponse = test::read_body_json(response).await;
}

#[sqlx::test(fixtures("sets", "cards"))]
pub async fn cards_list_can_be_filtered_by_language(db_pool: DbConnection) {
    let configuration = get_configuration().expect("Failed to build configuration.");

    let req = test::TestRequest::get()
        .uri(&add_query_parameters(
            "/api/cards",
            &mut vec![("language", "fr")],
        ))
        .insert_header(ContentType::json());

    let response = helpers::init_test_app_and_make_request(db_pool, configuration, req).await;
    assert!(response.status().is_success());

    let json: CardsListResponse = test::read_body_json(response).await;

    assert_ne!(0, json.meta.total);

    for card in json.data {
        assert_eq!("fr", card.language);
    }
}

#[sqlx::test(fixtures("sets", "cards"))]
pub async fn cards_list_can_be_filtered_by_name(db_pool: DbConnection) {
    let configuration = get_configuration().expect("Failed to build configuration.");

    let req = test::TestRequest::get()
        .uri(&add_query_parameters(
            "/api/cards",
            &mut vec![("name", "Deprivation")],
        ))
        .insert_header(ContentType::json());

    let response = helpers::init_test_app_and_make_request(db_pool, configuration, req).await;
    assert!(response.status().is_success());

    let json: CardsListResponse = test::read_body_json(response).await;

    assert_eq!(1, json.meta.total);

    for card in json.data {
        assert_eq!("Sensory Deprivation", card.name);
    }
}

#[sqlx::test(fixtures("sets", "cards"))]
pub async fn cards_list_can_be_filtered_by_rarity(db_pool: DbConnection) {
    let configuration = get_configuration().expect("Failed to build configuration.");

    let req = test::TestRequest::get()
        .uri(&add_query_parameters(
            "/api/cards",
            &mut vec![("rarity", "rare")],
        ))
        .insert_header(ContentType::json());

    let response = helpers::init_test_app_and_make_request(db_pool, configuration, req).await;
    assert!(response.status().is_success());

    let json: CardsListResponse = test::read_body_json(response).await;

    assert_eq!(2, json.meta.total);

    for card in json.data {
        assert_eq!(CardRarity::Rare, card.rarity);
    }
}
