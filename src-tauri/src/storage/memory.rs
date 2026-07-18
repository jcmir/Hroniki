use crate::domain::{Category, ChronicleObject, Entry};

use super::ChronologyRepository;

#[derive(Default)]
pub struct MemoryChronologyRepository {
    categories: Vec<Category>,
    objects: Vec<ChronicleObject>,
    entries: Vec<Entry>,
}

#[async_trait::async_trait]
impl ChronologyRepository for MemoryChronologyRepository {
    async fn save_category(&mut self, category: Category) -> Result<(), String> {
        self.categories.push(category);
        Ok(())
    }

    async fn save_object(&mut self, object: ChronicleObject) -> Result<(), String> {
        self.objects.push(object);
        Ok(())
    }

    async fn save_entry(&mut self, entry: Entry) -> Result<(), String> {
        self.entries.push(entry);
        Ok(())
    }

    async fn categories(&self) -> Result<Vec<Category>, String> {
        Ok(self.categories.clone())
    }

    async fn objects(&self) -> Result<Vec<ChronicleObject>, String> {
        Ok(self.objects.clone())
    }

    async fn entries(&self) -> Result<Vec<Entry>, String> {
        Ok(self.entries.clone())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn stores_category() {
        let mut repository = MemoryChronologyRepository::default();

        let category = Category::new("Garden").unwrap();

        repository.save_category(category).await.unwrap();

        assert_eq!(repository.categories().await.unwrap().len(), 1);
    }
}
