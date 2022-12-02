use chrono::NaiveDate;
use diesel::{insert_into, Insertable, QueryResult, Queryable, RunQueryDsl};
use uuid::Uuid;

use crate::domain::model::card::Card;
use crate::provider::card::CardProvider;
use crate::provider::database::set::DbSet;
use crate::provider::database::DbConnection;
use crate::schema::cards;
use crate::schema::cards::dsl::*;

#[derive(Insertable, Queryable)]
#[diesel(table_name = cards)]
pub struct DbCard {
    pub id: Uuid,
    pub scryfall_id: String,
    pub name: String,
    pub lang: String,
    pub released_at: NaiveDate,
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
        let _result = insert_into(cards)
            .values(cards_list)
            .execute(&mut connection);
    }

    fn get_cards(&self) -> Result<Vec<Card>, diesel::result::Error> {
        let mut connection = self.db_pool.get().unwrap();

        let result: QueryResult<Vec<DbCard>> = cards.load::<DbCard>(&mut connection);

        match result {
            Ok(db_cards) => {
                let list = db_cards.into_iter().map(|card| card.into()).collect();
                Ok(list)
            }
            Err(_) => Ok(vec![]), //TODO handle error cases
        }
    }
}
