use std::error::Error;
use std::fmt::Formatter;

pub struct DomainError(pub String);

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", &self.0)
    }
}

impl std::fmt::Debug for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", &self.0)
    }
}

impl Error for DomainError {}
