use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::{insert_into, Insertable, QueryResult, Queryable, RunQueryDsl};
use uuid::Uuid;

use crate::domain::model::card::{Card, CardRarity};
use crate::provider::card::CardProvider;
use crate::provider::database::DbConnection;
use crate::schema;

#[derive(Insertable, Queryable)]
#[diesel(table_name = schema::cards)]
pub struct DbCard {
    pub id: Uuid,
    pub scryfall_id: String,
    pub name: String,
    pub lang: String,
    pub released_at: NaiveDate,
    pub rarity: CardRarity,
    pub set_id: Uuid,
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
        }
    }
}

pub struct DbCardProvider<'a> {
    db_pool: &'a DbConnection,
}

impl<'a> DbCardProvider<'a> {
    pub fn new(db_pool: &'a DbConnection) -> Self {
        Self { db_pool }
    }
}

impl<'a> CardProvider for DbCardProvider<'a> {
    fn insert_cards(&self, cards_list: Vec<Card>) {
        let mut connection = self.db_pool.get().unwrap();

        let cards_list: Vec<DbCard> = cards_list.into_iter().map(|card| card.into()).collect();

        // TODO Yeah, I know, I will handle error cases
        let _result = insert_into(schema::cards::table)
            .values(cards_list)
            .execute(&mut connection);
    }

    fn get_cards(
        &self,
        language: Option<String>,
        name: Option<String>,
        rarity: Option<CardRarity>,
    ) -> Result<Vec<Card>, diesel::result::Error> {
        let mut connection = self.db_pool.get().unwrap();
        let mut query = schema::cards::table.into_boxed();

        if let Some(language) = language {
            query = query.filter(schema::cards::lang.eq(language));
        }
        if let Some(name) = name {
            query = query.filter(schema::cards::name.like(format!("%{}%", name)));
        }
        if let Some(rarity) = rarity {
            query = query.filter(schema::cards::rarity.eq(rarity));
        }

        let result: QueryResult<Vec<DbCard>> = query
            .order(schema::cards::name.asc())
            .load::<DbCard>(&mut connection);

        match result {
            Ok(db_cards) => {
                let list = db_cards.into_iter().map(|card| card.into()).collect();
                Ok(list)
            }
            Err(_) => Ok(vec![]), //TODO handle error cases
        }
    }
}
