use chrono::NaiveDate;
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use crate::domain::model::card::{Card, CardRarity};
use crate::provider::card::CardFilterParameters;
use crate::provider::database::DbConnection;
use crate::routes::Pagination;

#[derive(sqlx::FromRow)]
pub struct DbCard {
    pub id: Uuid,
    pub scryfall_id: String,
    pub name: String,
    pub lang: String,
    pub released_at: NaiveDate,
    pub rarity: CardRarity,
    pub set_id: Uuid,
    pub preview_image: Option<String>,
}

impl From<Card> for DbCard {
    fn from(card: Card) -> Self {
        Self {
            id: card.id,
            scryfall_id: card.scryfall_id,
            name: card.name,
            lang: card.lang,
            released_at: card.released_at,
            set_id: card.set_id,
            rarity: card.rarity,
            preview_image: card.preview_image,
        }
    }
}

impl Into<Card> for DbCard {
    fn into(self) -> Card {
        Card {
            id: self.id,
            scryfall_id: self.scryfall_id,
            name: self.name,
            lang: self.lang,
            released_at: self.released_at,
            set_id: self.set_id,
            rarity: self.rarity,
            preview_image: self.preview_image,
        }
    }
}

pub async fn insert_cards(db_pool: &DbConnection, cards_list: Vec<Card>) {
    let cards_list: Vec<DbCard> = cards_list.into_iter().map(|card| card.into()).collect();

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "INSERT INTO cards (id, scryfall_id, name, lang, released_at, set_id, rarity, preview_image) ",
    );

    query_builder.push_values(cards_list, |mut builder, card| {
        builder
            .push_bind(card.id)
            .push_bind(card.scryfall_id)
            .push_bind(card.name)
            .push_bind(card.lang)
            .push_bind(card.released_at)
            .push_bind(card.set_id)
            .push_bind(card.rarity)
            .push_bind(card.preview_image);
    });

    let _result = query_builder
        .build()
        .execute(db_pool)
        .await
        .expect("No error okay");
}

pub async fn get_cards(
    db_pool: &DbConnection,
    filters: CardFilterParameters,
    pagination: &Pagination,
) -> Result<Vec<Card>, sqlx::Error> {
    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("SELECT * FROM cards WHERE TRUE");

    if let Some(language) = filters.language {
        query_builder.push(" AND lang = ");
        query_builder.push_bind(language);
    }
    if let Some(name) = filters.name {
        query_builder.push(" AND name LIKE ");
        query_builder.push_bind(format!("%{}%", name));
    }
    if let Some(rarity) = filters.rarity {
        query_builder.push(" AND rarity = ");
        query_builder.push_bind(rarity);
    }

    query_builder
        .push(" ORDER BY name ASC LIMIT ")
        .push_bind(pagination.get_size() as i32)
        .push(" OFFSET ")
        .push_bind(pagination.get_offset() as i32);

    let result = query_builder
        .build_query_as::<DbCard>()
        .fetch_all(db_pool)
        .await;

    match result {
        Ok(db_cards) => {
            let list = db_cards.into_iter().map(|card| card.into()).collect();
            Ok(list)
        }
        Err(_) => Ok(vec![]), //TODO handle error cases
    }
}
