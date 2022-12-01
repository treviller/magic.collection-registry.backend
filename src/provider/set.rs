use crate::domain::model::set::Set;

pub trait SetProvider {
    fn get_all_sets(&self) -> Result<Vec<Set>, diesel::result::Error>;

    fn insert_sets(&self, sets_list: Vec<Set>);
}
