use chrono::NaiveDate;
use diesel::result::Error;
use diesel::{insert_into, PgConnection, RunQueryDsl};
use uuid::Uuid;

use magic_collection_registry_backend::provider::database::card::DbCard;
use magic_collection_registry_backend::schema::cards::dsl::*;

use crate::helpers::fixtures::set::SET_TEST_ID_1;
use crate::helpers::fixtures::Fixture;

pub struct CardFixtures;

impl CardFixtures {
    fn create_card(
        set_name: &str,
        set_set_id: Uuid,
        set_released_at: &str,
        set_lang: &str,
    ) -> DbCard {
        DbCard {
            id: Uuid::new_v4(),
            name: set_name.into(),
            set_id: set_set_id,
            released_at: NaiveDate::parse_from_str(&set_released_at, "%Y-%m-%d").unwrap(),
            lang: set_lang.into(),
            scryfall_id: Uuid::new_v4().to_string(),
        }
    }
}

impl Fixture for CardFixtures {
    fn load(connection: &mut PgConnection, _test_password_hash: &str) -> Result<(), Error> {
        let cards_list = vec![
            CardFixtures::create_card("Static Orb", *SET_TEST_ID_1, "2001-04-11", "en"),
            CardFixtures::create_card("Orbe Statique", *SET_TEST_ID_1, "2001-04-11", "fr"),
            CardFixtures::create_card("Sensory Deprivation", *SET_TEST_ID_1, "2001-04-11", "en"),
        ];

        for card in cards_list.iter() {
            insert_into(cards).values(card).execute(connection)?;
        }

        Ok(())
    }
}
