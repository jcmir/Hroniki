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

    pub async fn create_category(
        &mut self,
        name: impl Into<String>,
    ) -> Result<Category, DomainError> {
        let category = Category::new(name)?;

        self.repository
            .save_category(category.clone())
            .await
            .map_err(DomainError::StorageError)?;

        Ok(category)
    }

    pub async fn create_object(
        &mut self,
        category: &Category,
        name: impl Into<String>,
        description: Option<String>,
    ) -> Result<ChronicleObject, DomainError> {
        let object = ChronicleObject::new(category.id, name, description)?;

        self.repository
            .save_object(object.clone())
            .await
            .map_err(DomainError::StorageError)?;

        Ok(object)
    }

    pub async fn create_entry(
        &mut self,
        object: &ChronicleObject,
        title: impl Into<String>,
        description: Option<String>,
    ) -> Result<Entry, DomainError> {
        let entry = Entry::new(object.id, chrono::Utc::now(), title, description)?;

        self.repository
            .save_entry(entry.clone())
            .await
            .map_err(DomainError::StorageError)?;

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

    #[tokio::test]
    async fn creates_and_stores_full_history() {
        let repository = MemoryChronologyRepository::default();

        let mut service = ChronologyService::new(repository);

        let category = service.create_category("Garden").await.unwrap();

        let object = service
            .create_object(&category, "Apple tree", None)
            .await
            .unwrap();

        let entry = service
            .create_entry(&object, "Treatment", None)
            .await
            .unwrap();

        assert_eq!(service.repository().categories().await.unwrap().len(), 1);

        assert_eq!(service.repository().objects().await.unwrap().len(), 1);

        assert_eq!(service.repository().entries().await.unwrap().len(), 1);

        assert_eq!(entry.object_id, object.id);
    }
}
