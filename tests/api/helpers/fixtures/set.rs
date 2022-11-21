use chrono::NaiveDate;
use diesel::result::Error;
use diesel::{insert_into, PgConnection, RunQueryDsl};
use uuid::Uuid;

use magic_collection_registry_backend::domain::model::set::SetType;
use magic_collection_registry_backend::provider::database::set::DbSet;
use magic_collection_registry_backend::schema::sets::dsl::*;

use crate::helpers::fixtures::Fixture;

pub struct SetFixtures;

impl SetFixtures {
    fn create_set(
        set_code: &str,
        set_name: &str,
        set_set_type: SetType,
        set_released_at: &str,
        set_block: Option<String>,
        set_block_code: Option<String>,
        set_parent_set_id: Option<Uuid>,
        set_card_count: i32,
        set_printed_size: i32,
        set_foil_only: bool,
        set_non_foil_only: bool,
        set_icon_svg_uri: &str,
    ) -> DbSet {
        DbSet {
            id: Uuid::new_v4(),
            code: set_code.into(),
            name: set_name.into(),
            set_type: set_set_type,
            released_at: NaiveDate::parse_from_str(&set_released_at, "%Y-%m-%d").unwrap(),
            block_code: set_block_code,
            block: set_block,
            parent_set_id: set_parent_set_id,
            card_count: set_card_count,
            printed_size: set_printed_size,
            foil_only: set_foil_only,
            non_foil_only: set_non_foil_only,
            icon_svg_uri: set_icon_svg_uri.into(),
        }
    }
}

impl Fixture for SetFixtures {
    fn load(connection: &mut PgConnection, _test_password_hash: &str) -> Result<(), Error> {
        let sets_list: Vec<DbSet> = vec![SetFixtures::create_set(
            "aer",
            "Aether Revolt",
            SetType::Expansion,
            "2017-01-20",
            Some("kld".into()),
            Some("Kaladesh".into()),
            None,
            194,
            184,
            false,
            false,
            "https://svgs.scryfall.io/sets/aer.svg?1668402000",
        )];

        for set in sets_list.iter() {
            insert_into(sets).values(set).execute(connection)?;
        }

        Ok(())
    }
}
