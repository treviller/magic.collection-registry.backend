use chrono::NaiveDate;
use diesel::result::Error;
use diesel::{insert_into, PgConnection, RunQueryDsl};
use uuid::Uuid;

use magic_collection_registry_backend::domain::model::card::CardRarity;
use magic_collection_registry_backend::provider::database::card::DbCard;
use magic_collection_registry_backend::schema;

use crate::helpers::fixtures::set::SET_TEST_ID_1;
use crate::helpers::fixtures::Fixture;

pub struct CardFixtures;

impl CardFixtures {
    fn create_card(
        name: &str,
        set_id: Uuid,
        released_at: &str,
        lang: &str,
        rarity: CardRarity,
    ) -> DbCard {
        DbCard {
            id: Uuid::new_v4(),
            name: name.into(),
            set_id,
            released_at: NaiveDate::parse_from_str(&released_at, "%Y-%m-%d").unwrap(),
            lang: lang.into(),
            scryfall_id: Uuid::new_v4().to_string(),
            rarity,
        }
    }
}

impl Fixture for CardFixtures {
    fn load(connection: &mut PgConnection, _test_password_hash: &str) -> Result<(), Error> {
        let cards_list = vec![
            CardFixtures::create_card(
                "Static Orb",
                *SET_TEST_ID_1,
                "2001-04-11",
                "en",
                CardRarity::Rare,
            ),
            CardFixtures::create_card(
                "Orbe Statique",
                *SET_TEST_ID_1,
                "2001-04-11",
                "fr",
                CardRarity::Rare,
            ),
            CardFixtures::create_card(
                "Sensory Deprivation",
                *SET_TEST_ID_1,
                "2001-04-11",
                "en",
                CardRarity::Common,
            ),
        ];

        for card in cards_list.iter() {
            insert_into(schema::cards::table)
                .values(card)
                .execute(connection)?;
        }

        Ok(())
    }
}
