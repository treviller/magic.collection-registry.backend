use actix_web::{get, web, HttpResponse};

use crate::domain::card::CardService;
use crate::domain::model::card::CardRarity;
use crate::errors::api::card_list::CardListError;
use crate::provider::card::CardFilterParameters;
use crate::provider::database::DbConnection;
use crate::routes::responses::cards::CardsListResponse;
use crate::routes::Pagination;

#[derive(serde::Deserialize)]
pub struct QueryParameters {
    pub language: Option<String>,
    pub name: Option<String>,
    pub rarity: Option<CardRarity>,
    pub page: Option<String>,
    pub limit: Option<u32>,
}

impl From<QueryParameters> for CardFilterParameters {
    fn from(parameters: QueryParameters) -> Self {
        Self {
            name: parameters.name,
            language: parameters.language,
            rarity: parameters.rarity,
        }
    }
}

#[get("/cards")]
pub async fn list_cards(
    db_pool: web::Data<DbConnection>,
    parameters: web::Query<QueryParameters>,
) -> Result<HttpResponse, CardListError> {
    let pagination = Pagination::parse(parameters.page.clone(), parameters.limit)
        .map_err(CardListError::ValidationError)?;
    let card_service = CardService::new(&db_pool);
    let cards = card_service
        .list_cards(parameters.into_inner().into(), &pagination)
        .unwrap();

    Ok(HttpResponse::Ok().json(CardsListResponse::new(
        cards.into_iter().map(|card| card.into()).collect(),
        pagination,
    )))
}
