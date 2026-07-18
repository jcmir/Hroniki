use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    EmptyField(&'static str),
    StorageError(String),
}

impl Display for DomainError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyField(field) => {
                write!(formatter, "field `{field}` must not be empty")
            }
            Self::StorageError(message) => {
                write!(formatter, "storage error: {message}")
            }
        }
    }
}

impl std::error::Error for DomainError {}
