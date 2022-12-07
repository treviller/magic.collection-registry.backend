use serde::{Serialize, Serializer};

pub mod authentication;
pub mod cards;
pub mod responses;
pub mod sets;

#[derive(Clone, serde::Deserialize)]
pub struct Pagination {
    page: String,
    page_as_number: u32,
    size: u32,
}

impl Pagination {
    pub fn parse(page_value: Option<String>, size_value: Option<u32>) -> Result<Self, String> {
        let page = match page_value {
            Some(value) => value,
            None => "1".into(),
        };

        let page_as_number = page
            .parse::<u32>()
            .map_err(|e| format!("An error occurred during the parsing of page id : {}", e))?;

        let size = match size_value {
            Some(value) => value,
            None => 20,
        };

        Ok(Self {
            page,
            page_as_number,
            size,
        })
    }

    pub fn get_offset(&self) -> u32 {
        (self.page_as_number - 1) * self.size
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn get_next_page(&self) -> Option<String> {
        let next_value = self.page_as_number + 1;

        Some(next_value.to_string())
    }

    pub fn get_previous_page(&self) -> Option<String> {
        match self.page_as_number {
            1 => None,
            value => {
                let previous_value = value - 1;
                Some(previous_value.to_string())
            }
        }
    }
}

impl From<Pagination> for String {
    fn from(pagination: Pagination) -> Self {
        pagination.page
    }
}

impl Serialize for Pagination {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.page)
    }
}
