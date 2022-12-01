use crate::domain::model::card::Card;

pub trait CardProvider {
    fn insert_cards(&self, cards_list: Vec<Card>);
}
