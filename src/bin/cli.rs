use std::fmt::Formatter;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::time::SystemTime;

use chrono::NaiveDate;
use dotenvy::dotenv;
use serde::de::DeserializeOwned;
use serde_json::Deserializer;
use uuid::Uuid;

use magic_collection_registry_backend::domain::card::CardService;
use magic_collection_registry_backend::domain::model::card::Card;
use magic_collection_registry_backend::provider::database::establish_connection_pool;

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    bench(iterate_on_cards).await
}

fn download_file() -> String {
    "./cards-full.json".into()
}

#[derive(serde::Deserialize)]
struct DeserializedCard {
    arena_id: Option<i32>,
    object: String,
    id: String,
    oracle_id: Option<String>,
    multiverse_ids: Option<Vec<i32>>,
    mtgo_id: Option<i32>,
    mtgo_foil_id: Option<i32>,
    tcgplayer_id: Option<i32>,
    cardmarket_id: Option<i32>,
    name: String,
    lang: String,
    released_at: NaiveDate,
    uri: String,
    scryfall_uri: String,
    layout: String,
    highres_image: bool,
    image_status: String,
    image_uris: Option<ImageUris>,
    mana_cost: Option<String>,
    cmc: Option<f32>,
    type_line: Option<String>,
    oracle_text: Option<String>,
    colors: Option<Vec<String>>,
    color_identity: Vec<String>,
    keywords: Vec<String>,
    legalities: Legalities,
    games: Vec<String>,
    reserved: bool,
    foil: bool,
    nonfoil: bool,
    finishes: Vec<String>,
    oversized: bool,
    promo: bool,
    reprint: bool,
    variation: bool,
    set_id: String,
    set: String,
    set_name: String,
    set_type: String,
    set_uri: String,
    set_search_uri: String,
    scryfall_set_uri: String,
    rulings_uri: String,
    prints_search_uri: String,
    collector_number: String,
    digital: bool,
    rarity: String,
    flavor_text: Option<String>,
    card_back_id: Option<String>,
    artist: Option<String>,
    artist_ids: Option<Vec<String>>,
    illustration_id: Option<String>,
    border_color: String,
    frame: String,
    full_art: bool,
    textless: bool,
    booster: bool,
    story_spotlight: bool,
    edhrec_rank: Option<i32>,
    prices: Prices,
    related_uris: RelatedUris,
}

impl Into<Card> for DeserializedCard {
    fn into(self) -> Card {
        Card {
            id: Uuid::new_v4(),
            scryfall_id: self.id,
            name: self.name,
            lang: self.lang,
            released_at: self.released_at,
        }
    }
}

impl std::fmt::Debug for DeserializedCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "card : {}", self.name)
    }
}

#[derive(serde::Deserialize, Debug)]
struct ImageUris {
    small: String,
    normal: String,
    large: String,
    png: String,
    art_crop: String,
    border_crop: String,
}

#[derive(serde::Deserialize, Debug)]
struct Legalities {
    standard: String,
    future: String,
    historic: String,
    gladiator: String,
    pioneer: String,
    explorer: String,
    modern: String,
    legacy: String,
    pauper: String,
    vintage: String,
    penny: String,
    commander: String,
    brawl: String,
    historicbrawl: String,
    alchemy: String,
    paupercommander: String,
    duel: String,
    oldschool: String,
    premodern: String,
}

#[derive(serde::Deserialize, Debug)]
struct Prices {
    usd: Option<String>,
    usd_foil: Option<String>,
    usd_etched: Option<String>,
    eur: Option<String>,
    eur_foil: Option<String>,
    tix: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
struct RelatedUris {
    gatherer: Option<String>,
    tcgplayer_infinite_articles: Option<String>,
    tcgplayer_infinite_decks: Option<String>,
    edhrec: Option<String>,
}

async fn bench<F>(closure: F) -> Result<(), io::Error>
where
    F: FnOnce() -> Result<(), io::Error>,
{
    let start = SystemTime::now();
    let result = closure();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();

    if duration.as_secs_f64() < 1.0 {
        println!("it tooks {} ms", duration.as_millis());
    } else {
        println!("it tooks {} secs", duration.as_secs_f64());
    }

    result
}

struct CardLoader<'a> {
    cards: Vec<Card>,
    size: u16,
    max_size: u16,
    total_count: u32,
    card_service: CardService<'a>,
}

impl<'a> CardLoader<'a> {
    pub fn new(max_size: u16, card_service: CardService<'a>) -> Self {
        Self {
            cards: vec![],
            size: 0,
            total_count: 0,
            max_size,
            card_service,
        }
    }

    pub fn add(&mut self, card: DeserializedCard) {
        self.cards.push(card.into());
        self.size += 1;
        self.total_count += 1;

        if self.size >= self.max_size {
            self.flush();
        }
    }

    pub fn get_last_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn terminate(&mut self) {
        println!("{} cards loaded.", self.total_count);

        if self.size > 0 {
            self.flush()
        }
    }

    fn flush(&mut self) {
        self.card_service.add_cards(self.cards.drain(1..).collect());
        self.size = 0;
    }
}

fn iterate_on_cards() -> Result<(), io::Error> {
    let filepath = download_file();
    let reader = BufReader::new(File::open(filepath)?);

    let db_pool = establish_connection_pool();
    let mut cards_loader = CardLoader::new(500, CardService::new(&db_pool));

    for card in iter_on_json_array(reader) {
        let card = match card {
            Ok(card) => card,
            Err(e) => {
                println!("An error occurred during parsing : {}", e);

                if let Some(card) = cards_loader.get_last_card() {
                    println!("Last card name : {}", card.name);
                }

                panic!();
            }
        };

        cards_loader.add(card);
    }

    cards_loader.terminate();

    Ok(())
}

fn read_skipping_ws(mut reader: impl io::Read) -> io::Result<u8> {
    loop {
        let mut byte = 0u8;
        reader.read_exact(std::slice::from_mut(&mut byte))?;

        if !byte.is_ascii_whitespace() {
            return Ok(byte);
        }
    }
}

fn iter_on_json_array<T: DeserializeOwned, R: io::Read>(
    mut reader: R,
) -> impl Iterator<Item = Result<T, io::Error>> {
    let mut started = false;

    std::iter::from_fn(move || yield_next_object(&mut reader, &mut started).transpose())
}

fn yield_next_object<T: DeserializeOwned, R: io::Read>(
    mut reader: R,
    started: &mut bool,
) -> io::Result<Option<T>> {
    if !*started {
        *started = true;
        match read_skipping_ws(&mut reader)? {
            b'[' => deserialize_single(reader).map(Some),
            _ => Err(invalid_data("`[` not found")),
        }
    } else {
        match read_skipping_ws(&mut reader)? {
            b',' => deserialize_single(reader).map(Some),
            b']' => Ok(None),
            _ => Err(invalid_data("`,` or `]` not found")),
        }
    }
}

fn invalid_data(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, msg)
}

fn deserialize_single<T: DeserializeOwned, R: io::Read>(reader: R) -> io::Result<T> {
    let object = Deserializer::from_reader(reader).into_iter::<T>().next();

    match object {
        Some(result) => result.map_err(Into::into),
        None => Err(invalid_data("premature EOF")),
    }
}
