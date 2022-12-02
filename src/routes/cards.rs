use actix_web::{get, web, HttpResponse};

use crate::domain::card::CardService;
use crate::domain::model::card::CardRarity;
use crate::errors::api::card_list::CardListError;
use crate::provider::database::DbConnection;
use crate::routes::responses::cards::CardsListResponse;
use crate::routes::PageId;

#[derive(serde::Deserialize)]
pub struct QueryParameters {
    language: Option<String>,
    name: Option<String>,
    rarity: Option<CardRarity>,
    page: Option<String>,
    limit: Option<u16>,
}

#[get("/cards")]
pub async fn list_cards(
    db_pool: web::Data<DbConnection>,
    parameters: web::Query<QueryParameters>,
) -> Result<HttpResponse, CardListError> {
    let current_page =
        PageId::parse(parameters.page.clone()).map_err(CardListError::ValidationError)?;
    let card_service = CardService::new(&db_pool);
    let cards = card_service
        .list_cards(
            parameters.language.clone(),
            parameters.name.clone(),
            parameters.rarity.clone(),
        )
        .unwrap();

    Ok(HttpResponse::Ok().json(CardsListResponse::new(
        cards.into_iter().map(|card| card.into()).collect(),
        current_page,
    )))
}
