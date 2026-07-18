use crate::{
    domain::{Category, ChronicleObject, DomainError, Entry},
    storage::ChronologyRepository,
};

pub struct ChronologyService<R>
where
    R: ChronologyRepository,
{
    repository: R,
}

impl<R> ChronologyService<R>
where
    R: ChronologyRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn create_category(&mut self, name: impl Into<String>) -> Result<Category, DomainError> {
        let category = Category::new(name)?;

        self.repository.save_category(category.clone());

        Ok(category)
    }

    pub fn create_object(
        &mut self,
        category: &Category,
        name: impl Into<String>,
        description: Option<String>,
    ) -> Result<ChronicleObject, DomainError> {
        let object = ChronicleObject::new(category.id, name, description)?;

        self.repository.save_object(object.clone());

        Ok(object)
    }

    pub fn create_entry(
        &mut self,
        object: &ChronicleObject,
        title: impl Into<String>,
        description: Option<String>,
    ) -> Result<Entry, DomainError> {
        let entry = Entry::new(object.id, chrono::Utc::now(), title, description)?;

        self.repository.save_entry(entry.clone());

        Ok(entry)
    }

    pub fn repository(&self) -> &R {
        &self.repository
    }

    pub fn repository_mut(&mut self) -> &mut R {
        &mut self.repository
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::storage::MemoryChronologyRepository;

    #[test]
    fn creates_and_stores_full_history() {
        let repository = MemoryChronologyRepository::default();

        let mut service = ChronologyService::new(repository);

        let category = service.create_category("Garden").unwrap();

        let object = service
            .create_object(&category, "Apple tree", None)
            .unwrap();

        let entry = service.create_entry(&object, "Treatment", None).unwrap();

        assert_eq!(service.repository().categories().len(), 1);

        assert_eq!(service.repository().objects().len(), 1);

        assert_eq!(service.repository().entries().len(), 1);

        assert_eq!(entry.object_id, object.id);
    }
}
