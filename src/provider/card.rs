use crate::domain::model::card::Card;

pub trait CardProvider {
    fn insert_cards(&self, cards_list: Vec<Card>);

    fn get_cards(&self) -> Result<Vec<Card>, diesel::result::Error>;
}
