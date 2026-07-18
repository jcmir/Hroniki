use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    EmptyField(&'static str),
}

impl Display for DomainError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyField(field) => {
                write!(formatter, "field `{field}` must not be empty")
            }
        }
    }
}

impl std::error::Error for DomainError {}
