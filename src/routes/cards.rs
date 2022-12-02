use actix_web::{get, web, HttpResponse};

use crate::domain::card::CardService;
use crate::provider::database::DbConnection;
use crate::routes::responses::cards::CardsListResponse;

#[derive(serde::Deserialize)]
pub struct QueryParameters {
    language: Option<String>,
}

#[get("/cards")]
pub async fn list_cards(
    db_pool: web::Data<DbConnection>,
    parameters: web::Query<QueryParameters>,
) -> HttpResponse {
    let card_service = CardService::new(&db_pool);
    let cards = card_service
        .list_cards(parameters.language.clone())
        .unwrap();

    HttpResponse::Ok().json(CardsListResponse::new(
        cards.into_iter().map(|card| card.into()).collect(),
    ))
}
