use crate::domain::{Category, ChronicleObject, DomainError, Entry};

pub struct ChronologyService;

impl ChronologyService {
    pub fn create_category(name: impl Into<String>) -> Result<Category, DomainError> {
        Category::new(name)
    }

    pub fn create_object(
        category: &Category,
        name: impl Into<String>,
        description: Option<String>,
    ) -> Result<ChronicleObject, DomainError> {
        ChronicleObject::new(category.id, name, description)
    }

    pub fn create_entry(
        object: &ChronicleObject,
        title: impl Into<String>,
        description: Option<String>,
    ) -> Result<Entry, DomainError> {
        Entry::new(object.id, chrono::Utc::now(), title, description)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_full_chronology_chain() {
        let category = ChronologyService::create_category("Garden").unwrap();

        let object = ChronologyService::create_object(&category, "Apple tree", None).unwrap();

        let entry = ChronologyService::create_entry(&object, "First treatment", None).unwrap();

        assert_eq!(entry.object_id, object.id);
    }
}
