use crate::domain::{Category, ChronicleObject, Entry};

#[async_trait::async_trait]
pub trait ChronologyRepository {
    async fn save_category(&mut self, category: Category) -> Result<(), String>;

    async fn save_object(&mut self, object: ChronicleObject) -> Result<(), String>;

    async fn save_entry(&mut self, entry: Entry) -> Result<(), String>;

    async fn categories(&self) -> Result<Vec<Category>, String>;

    async fn objects(&self) -> Result<Vec<ChronicleObject>, String>;

    async fn entries(&self) -> Result<Vec<Entry>, String>;
}
