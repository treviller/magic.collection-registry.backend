use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::provider::scryfall::api::ScryfallListResponse;

#[derive(serde::Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ScryfallBulkDataType {
    OracleCards,
    UniqueArtwork,
    Rulings,
    DefaultCards,
    AllCards,
}

#[derive(serde::Deserialize)]
pub struct ScryfallBulkDataFile {
    #[serde(rename = "type")]
    pub data_type: ScryfallBulkDataType,
    pub download_uri: String,
}

pub struct FileDownloader {
    file_path: String,
    client: reqwest::Client,
}

impl FileDownloader {
    pub fn new(filename: String) -> Self {
        Self {
            file_path: format!("./{}", filename),
            client: reqwest::Client::new(),
        }
    }

    pub async fn download_file(&self) -> String {
        println!("Start to download cards file");

        let uri = self.get_cards_file_download_uri().await;

        println!("Downloading all cards file from this uri : {}", uri);

        let mut response = self.client.get(uri).send().await.unwrap();
        let mut file = self.create_cards_file();

        while let Some(chunk) = response
            .chunk()
            .await
            .expect("An error occurred while waiting chunks")
        {
            file.write(&chunk)
                .expect("An error occurred while writing into file");
        }

        self.file_path.clone()
    }

    fn create_cards_file(&self) -> File {
        let path = Path::new(&self.file_path);

        match OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&path)
        {
            Ok(file) => file,
            Err(err) => panic!("Error when creating file : {}", err),
        }
    }

    async fn get_cards_file_download_uri(&self) -> String {
        let url = "https://api.scryfall.com/bulk-data";
        let response = self.client.get(url).send().await.unwrap();

        let scryfall_bulk_data_files: ScryfallListResponse<ScryfallBulkDataFile> =
            response.json().await.unwrap();

        for file in scryfall_bulk_data_files.data {
            if file.data_type == ScryfallBulkDataType::AllCards {
                return file.download_uri;
            }
        }

        panic!("Unable to find all cards file from Scryfall's bulk data list endpoint");
    }
}
