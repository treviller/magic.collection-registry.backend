use std::fmt::Formatter;

use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::model::card::Card;
use crate::domain::model::card::CardRarity;

#[derive(serde::Deserialize)]
pub struct ScryfallCard {
    // _arena_id: Option<i32>,
    // _object: String,
    id: String,
    // _oracle_id: Option<String>,
    // _multiverse_ids: Option<Vec<i32>>,
    // _mtgo_id: Option<i32>,
    // _mtgo_foil_id: Option<i32>,
    // _tcgplayer_id: Option<i32>,
    // _cardmarket_id: Option<i32>,
    name: String,
    printed_name: Option<String>,
    lang: String,
    released_at: NaiveDate,
    // _uri: String,
    // _scryfall_uri: String,
    // _layout: String,
    // _highres_image: bool,
    // _image_status: String,
    image_uris: Option<ImageUris>,
    // _mana_cost: Option<String>,
    // _cmc: Option<f32>,
    // _type_line: Option<String>,
    // _oracle_text: Option<String>,
    // _colors: Option<Vec<String>>,
    // _color_identity: Vec<String>,
    // _keywords: Vec<String>,
    // _legalities: Legalities,
    // _games: Vec<String>,
    // _reserved: bool,
    // _foil: bool,
    // _nonfoil: bool,
    // _finishes: Vec<String>,
    // _oversized: bool,
    // _promo: bool,
    // _reprint: bool,
    // _variation: bool,
    set_id: Uuid,
    // _set: String,
    // _set_name: String,
    // _set_type: String,
    // _set_uri: String,
    // _set_search_uri: String,
    // _scryfall_set_uri: String,
    // _rulings_uri: String,
    // _prints_search_uri: String,
    // _collector_number: String,
    // _digital: bool,
    rarity: CardRarity,
    // _flavor_text: Option<String>,
    // _card_back_id: Option<String>,
    // _artist: Option<String>,
    // _artist_ids: Option<Vec<String>>,
    // _illustration_id: Option<String>,
    // _border_color: String,
    // _frame: String,
    // _full_art: bool,
    // _textless: bool,
    // _booster: bool,
    // _story_spotlight: bool,
    // _edhrec_rank: Option<i32>,
    // _prices: Prices,
    // _related_uris: RelatedUris,
}

impl Into<Card> for ScryfallCard {
    fn into(self) -> Card {
        let card_name = match self.printed_name {
            Some(name) => name,
            None => self.name,
        };

        Card {
            id: Uuid::new_v4(),
            scryfall_id: self.id,
            name: card_name,
            lang: self.lang,
            released_at: self.released_at,
            set_id: self.set_id,
            rarity: self.rarity,
            preview_image: self.image_uris.map(|images| images.normal),
        }
    }
}

impl std::fmt::Debug for ScryfallCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "card : {}", self.name)
    }
}

#[derive(serde::Deserialize, Debug)]
struct ImageUris {
    // _small: String,
    normal: String,
    // _large: String,
    // _png: String,
    // _art_crop: String,
    // _border_crop: String,
}

#[derive(serde::Deserialize, Debug)]
struct Legalities {
    _standard: String,
    _future: String,
    _historic: String,
    _gladiator: String,
    _pioneer: String,
    _explorer: String,
    _modern: String,
    _legacy: String,
    _pauper: String,
    _vintage: String,
    _penny: String,
    _commander: String,
    _brawl: String,
    _historicbrawl: String,
    _alchemy: String,
    _paupercommander: String,
    _duel: String,
    _oldschool: String,
    _premodern: String,
}

#[derive(serde::Deserialize, Debug)]
struct Prices {
    _usd: Option<String>,
    _usd_foil: Option<String>,
    _usd_etched: Option<String>,
    _eur: Option<String>,
    _eur_foil: Option<String>,
    _tix: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
struct RelatedUris {
    _gatherer: Option<String>,
    _tcgplayer_infinite_articles: Option<String>,
    _tcgplayer_infinite_decks: Option<String>,
    _edhrec: Option<String>,
}
