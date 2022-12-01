use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::{insert_into, AsChangeset, Identifiable, Insertable, Queryable, RunQueryDsl};
use uuid::Uuid;

use crate::domain::model::set::{Set, SetCode, SetType};
use crate::provider::database::DbConnection;
use crate::provider::set::SetProvider;
use crate::schema::sets;
use crate::schema::sets::dsl::*;

#[derive(Queryable, Identifiable, AsChangeset, Insertable)]
#[diesel(table_name = sets)]
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

pub struct DbSetProvider<'a> {
    db_pool: &'a DbConnection,
}

impl<'a> DbSetProvider<'a> {
    pub fn new(db_pool: &'a DbConnection) -> Self {
        Self { db_pool }
    }
}

impl<'a> SetProvider for DbSetProvider<'a> {
    fn get_all_sets(&self) -> Result<Vec<Set>, diesel::result::Error> {
        let mut connection = self.db_pool.get().unwrap();

        let result: QueryResult<Vec<DbSet>> = sets.load::<DbSet>(&mut connection);

        match result {
            Ok(db_sets) => {
                let list = db_sets.into_iter().map(|set| set.into()).collect();
                Ok(list)
            }
            Err(_) => Ok(vec![]), //TODO handle error cases
        }
    }

    fn insert_sets(&self, sets_list: Vec<Set>) {
        let mut connection = self.db_pool.get().unwrap();

        let sets_list: Vec<DbSet> = sets_list.into_iter().map(|set| set.into()).collect();

        //TODO handle error cases
        let _result = insert_into(sets).values(sets_list).execute(&mut connection);
    }
}
