use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{CategoryId, ChronicleObjectId, DomainError, EntryId, PhotoId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Category {
    pub id: CategoryId,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl Category {
    pub fn new(name: impl Into<String>) -> Result<Self, DomainError> {
        let name = validate_required_text(name.into(), "category.name")?;

        Ok(Self {
            id: CategoryId::new(),
            name,
            created_at: Utc::now(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronicleObject {
    pub id: ChronicleObjectId,
    pub category_id: CategoryId,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl ChronicleObject {
    pub fn new(
        category_id: CategoryId,
        name: impl Into<String>,
        description: Option<String>,
    ) -> Result<Self, DomainError> {
        let name = validate_required_text(name.into(), "object.name")?;
        let description = normalize_optional_text(description);

        Ok(Self {
            id: ChronicleObjectId::new(),
            category_id,
            name,
            description,
            created_at: Utc::now(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    pub id: EntryId,
    pub object_id: ChronicleObjectId,
    pub occurred_at: DateTime<Utc>,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Entry {
    pub fn new(
        object_id: ChronicleObjectId,
        occurred_at: DateTime<Utc>,
        title: impl Into<String>,
        description: Option<String>,
    ) -> Result<Self, DomainError> {
        let title = validate_required_text(title.into(), "entry.title")?;
        let description = normalize_optional_text(description);
        let now = Utc::now();

        Ok(Self {
            id: EntryId::new(),
            object_id,
            occurred_at,
            title,
            description,
            created_at: now,
            updated_at: now,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Photo {
    pub id: PhotoId,
    pub entry_id: EntryId,
    pub path: String,
    pub thumbnail: String,
    pub created_at: DateTime<Utc>,
}

impl Photo {
    pub fn new(
        entry_id: EntryId,
        path: impl Into<String>,
        thumbnail: impl Into<String>,
    ) -> Self {
        Self {
            id: PhotoId::new(),
            entry_id,
            path: path.into(),
            thumbnail: thumbnail.into(),
            created_at: Utc::now(),
        }
    }
}


fn validate_required_text(value: String, field: &'static str) -> Result<String, DomainError> {
    let value = value.trim();

    if value.is_empty() {
        return Err(DomainError::EmptyField(field));
    }

    Ok(value.to_owned())
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let value = value.trim();

        if value.is_empty() {
            None
        } else {
            Some(value.to_owned())
        }
    })
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[test]
    fn category_rejects_empty_name() {
        let result = Category::new("   ");

        assert_eq!(result, Err(DomainError::EmptyField("category.name")));
    }

    #[test]
    fn category_trims_name() {
        let category = Category::new("  Garden  ").expect("category should be valid");

        assert_eq!(category.name, "Garden");
    }

    #[test]
    fn chronicle_object_belongs_to_category() {
        let category = Category::new("Garden").expect("category should be valid");

        let object = ChronicleObject::new(
            category.id,
            "Apple tree",
            Some("  Tree near the house  ".to_owned()),
        )
        .expect("object should be valid");

        assert_eq!(object.category_id, category.id);
        assert_eq!(object.name, "Apple tree");
        assert_eq!(object.description.as_deref(), Some("Tree near the house"));
    }

    #[test]
    fn entry_belongs_to_object() {
        let category = Category::new("Garden").expect("category should be valid");

        let object =
            ChronicleObject::new(category.id, "Apple tree", None).expect("object should be valid");

        let entry = Entry::new(
            object.id,
            Utc::now(),
            "Treated against fungus",
            Some("  First treatment  ".to_owned()),
        )
        .expect("entry should be valid");

        assert_eq!(entry.object_id, object.id);
        assert_eq!(entry.title, "Treated against fungus");
        assert_eq!(entry.description.as_deref(), Some("First treatment"));
    }

    #[test]
    fn blank_optional_description_becomes_none() {
        let category = Category::new("Garden").expect("category should be valid");

        let object = ChronicleObject::new(category.id, "Apple tree", Some("   ".to_owned()))
            .expect("object should be valid");

        assert_eq!(object.description, None);
    }
}
