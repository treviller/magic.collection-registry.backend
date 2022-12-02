use serde::{Serialize, Serializer};

pub mod authentication;
pub mod cards;
pub mod responses;
pub mod sets;

#[derive(Clone, serde::Deserialize)]
pub struct PageId {
    value: String,
    value_as_number: u32,
}

impl PageId {
    pub fn parse(value: Option<String>) -> Result<Self, String> {
        let value = match value {
            Some(value) => value,
            None => "1".into(),
        };

        let value_as_number = value
            .parse::<u32>()
            .map_err(|e| format!("An error occurred during the parsing of page id : {}", e))?;

        Ok(Self {
            value,
            value_as_number,
        })
    }

    pub fn as_u32(&self) -> u32 {
        self.value_as_number
    }

    pub fn get_next_page(&self) -> Option<String> {
        let next_value = self.value_as_number + 1;

        Some(next_value.to_string())
    }

    pub fn get_previous_page(&self) -> Option<String> {
        match self.value_as_number {
            1 => None,
            value => {
                let previous_value = value - 1;
                Some(previous_value.to_string())
            }
        }
    }
}

impl Into<String> for PageId {
    fn into(self) -> String {
        self.value
    }
}

impl Serialize for PageId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.value)
    }
}
