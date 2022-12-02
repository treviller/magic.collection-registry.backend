use actix_web::{get, web, HttpResponse};

use crate::domain::card::CardService;
use crate::provider::database::DbConnection;
use crate::routes::responses::cards::CardsListResponse;

#[get("/cards")]
pub async fn list_cards(db_pool: web::Data<DbConnection>) -> HttpResponse {
    let card_service = CardService::new(&db_pool);
    let cards = card_service.list_cards().unwrap();

    HttpResponse::Ok().json(CardsListResponse::new(
        cards.into_iter().map(|card| card.into()).collect(),
    ))
}
