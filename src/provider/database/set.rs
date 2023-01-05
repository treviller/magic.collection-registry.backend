use chrono::NaiveDate;
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use crate::domain::model::set::{Set, SetCode, SetType};
use crate::provider::database::DbConnection;

#[derive(sqlx::FromRow)]
pub struct DbSet {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub set_type: SetType,
    pub released_at: NaiveDate,
    pub block_code: Option<String>,
    pub block: Option<String>,
    pub parent_set_id: Option<Uuid>,
    pub card_count: i32,
    pub printed_size: i32,
    pub foil_only: bool,
    pub non_foil_only: bool,
    pub icon_svg_uri: String,
}

impl From<Set> for DbSet {
    fn from(set: Set) -> Self {
        Self {
            id: set.id,
            code: set.code.into(),
            name: set.name,
            set_type: set.set_type,
            released_at: set.released_at,
            block_code: set.block_code,
            block: set.block,
            parent_set_id: set.parent_set_id,
            card_count: set.card_count.into(),
            printed_size: set.printed_size.into(),
            foil_only: set.foil_only,
            non_foil_only: set.non_foil_only,
            icon_svg_uri: set.icon_svg_uri,
        }
    }
}

impl Into<Set> for DbSet {
    fn into(self) -> Set {
        Set {
            id: self.id,
            code: SetCode::parse(self.code).expect("Set code from database should be valid."),
            name: self.name,
            set_type: self.set_type,
            released_at: self.released_at,
            block_code: self.block_code,
            block: self.block,
            parent_set_id: self.parent_set_id,
            card_count: self.card_count as u16,
            printed_size: self.printed_size as u16,
            foil_only: self.foil_only,
            non_foil_only: self.non_foil_only,
            icon_svg_uri: self.icon_svg_uri,
        }
    }
}

pub async fn get_all_sets(db_pool: &DbConnection) -> Result<Vec<Set>, sqlx::Error> {
    let result: sqlx::Result<Vec<DbSet>> = sqlx::query_as::<Postgres, DbSet>("SELECT * FROM sets")
        .fetch_all(db_pool)
        .await;

    match result {
        Ok(db_sets) => {
            let list = db_sets.into_iter().map(|set| set.into()).collect();
            Ok(list)
        }
        Err(_) => Ok(vec![]), //TODO handle error cases
    }
}

pub async fn insert_sets(db_pool: &DbConnection, sets_list: Vec<Set>) {
    let sets_list: Vec<DbSet> = sets_list.into_iter().map(|set| set.into()).collect();

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("INSERT INTO sets(id, code, name, set_type, released_at, block_code, block, parent_set_id, card_count, printed_size, foil_only, non_foil_only, icon_svg_uri) ");

    query_builder.push_values(sets_list, |mut builder, set| {
        builder
            .push_bind(set.id)
            .push_bind(set.code)
            .push_bind(set.name)
            .push_bind(set.set_type)
            .push_bind(set.released_at)
            .push_bind(set.block_code)
            .push_bind(set.block)
            .push_bind(set.parent_set_id)
            .push_bind(set.card_count)
            .push_bind(set.printed_size)
            .push_bind(set.foil_only)
            .push_bind(set.non_foil_only)
            .push_bind(set.icon_svg_uri);
    });

    let _result = query_builder
        .build()
        .execute(db_pool)
        .await
        .expect("No error okay");
}
