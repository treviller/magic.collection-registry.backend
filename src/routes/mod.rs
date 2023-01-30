use serde::{Serialize, Serializer};

pub mod authentication;
pub mod cards;
pub mod responses;
pub mod sets;

#[derive(Clone, serde::Deserialize)]
pub struct PaginationParameters {
    page: String,
    page_as_number: i64,
    size: i64,
}

impl PaginationParameters {
    pub fn parse(page_value: Option<String>, size_value: Option<u64>) -> Result<Self, String> {
        let page = match page_value {
            Some(value) => value,
            None => "1".into(),
        };

        let page_as_number = page
            .parse::<u64>()
            .map_err(|e| format!("An error occurred during the parsing of page id : {}", e))?;

        let size = match size_value {
            Some(value) => value,
            None => 20,
        };

        Ok(Self {
            page,
            page_as_number: page_as_number as i64,
            size: size as i64,
        })
    }

    pub fn get_current_page_index(&self) -> i64 {
        self.page_as_number
    }

    pub fn get_offset(&self) -> i64 {
        (self.page_as_number - 1) * self.size
    }

    pub fn get_size(&self) -> i64 {
        self.size
    }
}

impl From<PaginationParameters> for String {
    fn from(pagination: PaginationParameters) -> Self {
        pagination.page
    }
}

impl Serialize for PaginationParameters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.page)
    }
}
