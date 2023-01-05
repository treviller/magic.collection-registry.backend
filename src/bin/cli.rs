use std::fs::File;
use std::future::Future;
use std::io;
use std::io::BufReader;
use std::time::SystemTime;

use dotenvy::dotenv;
use serde::de::DeserializeOwned;
use serde_json::Deserializer;

use magic_collection_registry_backend::domain::card::CardService;
use magic_collection_registry_backend::domain::model::card::Card;
use magic_collection_registry_backend::domain::model::set::Set;
use magic_collection_registry_backend::domain::set::SetService;
use magic_collection_registry_backend::provider::database::{
    establish_connection_pool, DbConnection,
};
use magic_collection_registry_backend::provider::scryfall::api::ScryfallSetListResponse;
use magic_collection_registry_backend::provider::scryfall::card::ScryfallCard;

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let db_pool = establish_connection_pool();

    bench(load_sets, "Loading sets", &db_pool)
        .await
        .expect("An error occurred when loading sets");
    bench(load_cards, "Loading cards", &db_pool).await
}

fn download_file() -> String {
    "./cards-full.json".into()
}

async fn bench<'a, C, F>(closure: C, name: &str, db_pool: &'a DbConnection) -> Result<(), io::Error>
where
    F: Future<Output = Result<(), io::Error>> + 'a,
    C: FnOnce(&'a DbConnection) -> F + 'a,
{
    let start = SystemTime::now();
    let result = closure(db_pool).await;
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();

    if duration.as_secs_f64() < 1.0 {
        println!("{} took {} ms", name, duration.as_millis());
    } else {
        println!("{} took {} secs", name, duration.as_secs_f64());
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

    pub async fn add(&mut self, card: ScryfallCard) {
        self.cards.push(card.into());
        self.size += 1;
        self.total_count += 1;

        if self.size >= self.max_size {
            self.flush().await;
        }
    }

    pub fn get_last_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub async fn terminate(&mut self) {
        println!("{} cards loaded.", self.total_count);

        if self.size > 0 {
            self.flush().await
        }
    }

    async fn flush(&mut self) {
        self.card_service
            .add_cards(self.cards.drain(1..).collect())
            .await;
        self.size = 0;
    }
}

async fn load_cards(db_pool: &DbConnection) -> Result<(), io::Error> {
    let filepath = download_file();
    let reader = BufReader::new(File::open(filepath)?);

    let mut cards_loader = CardLoader::new(500, CardService::new(db_pool));

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

        cards_loader.add(card).await;
    }

    cards_loader.terminate().await;

    Ok(())
}

async fn load_sets(db_pool: &DbConnection) -> Result<(), io::Error> {
    let url = "https://api.scryfall.com/sets";

    let client = reqwest::Client::new();
    let response = client.get(url).send().await.unwrap();
    let scryfall_sets: ScryfallSetListResponse = response.json().await.unwrap();

    let sets: Vec<Set> = scryfall_sets
        .data
        .into_iter()
        .map(|set| set.into_set().unwrap())
        .collect();

    let set_service = SetService::new(db_pool);

    set_service.add_sets(sets).await;

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
