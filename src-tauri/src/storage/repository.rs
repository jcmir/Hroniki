use crate::domain::{Category, ChronicleObject, Entry, EntryId, Photo};

#[async_trait::async_trait]
pub trait ChronologyRepository {
    async fn save_category(&mut self, category: Category) -> Result<(), String>;

    async fn save_object(&mut self, object: ChronicleObject) -> Result<(), String>;

    async fn save_entry(&mut self, entry: Entry) -> Result<(), String>;

    async fn save_photo(&mut self, photo: Photo) -> Result<(), String>;

    async fn delete_entry(&mut self, id: EntryId) -> Result<(), String>;

    async fn update_entry(&mut self, id: EntryId, title: String, description: Option<String>) -> Result<(), String>;

    async fn categories(&self) -> Result<Vec<Category>, String>;

    async fn objects(&self) -> Result<Vec<ChronicleObject>, String>;

    async fn entries(&self) -> Result<Vec<Entry>, String>;

    async fn entry_photos(&self, entry_id: EntryId) -> Result<Vec<Photo>, String>;
}
