use crate::domain::{Category, ChronicleObject, Entry, EntryId, Photo, Reminder};

#[async_trait::async_trait]
pub trait ChronologyRepository {
    async fn save_category(&mut self, category: Category) -> Result<(), String>;

    async fn save_object(&mut self, object: ChronicleObject) -> Result<(), String>;

    async fn save_entry(&mut self, entry: Entry) -> Result<(), String>;

    async fn save_entry_with_photos(&mut self, entry: Entry, photos: Vec<Photo>) -> Result<(), String>;

    async fn save_photo(&mut self, photo: Photo) -> Result<(), String>;

    async fn save_reminder(&mut self, reminder: Reminder) -> Result<(), String>;

    async fn delete_entry(&mut self, id: EntryId) -> Result<(), String>;

    async fn update_entry(&mut self, id: EntryId, title: String, description: Option<String>) -> Result<(), String>;

    async fn categories(&self) -> Result<Vec<Category>, String>;

    async fn objects(&self) -> Result<Vec<ChronicleObject>, String>;

    async fn entries(&self) -> Result<Vec<Entry>, String>;

    async fn entry_photos(&self, entry_id: EntryId) -> Result<Vec<Photo>, String>;

    async fn entry_reminders(&self, entry_id: EntryId) -> Result<Vec<Reminder>, String>;

    async fn reminders(&self) -> Result<Vec<Reminder>, String>;

    async fn search_entries(
        &self,
        query_text: Option<String>,
        category_id: Option<String>,
        object_id: Option<String>,
        start_date: Option<String>,
        end_date: Option<String>
    ) -> Result<Vec<Entry>, String>;

    async fn get_object_stats(&self, object_id: crate::domain::ChronicleObjectId) -> Result<ObjectStats, String>;
}

#[derive(serde::Serialize, Clone)]
pub struct ObjectStats {
    pub age_days: i64,
    pub total_entries: usize,
    pub total_photos: usize,
    pub last_event_title: Option<String>,
    pub last_event_date: Option<String>,
    pub next_reminder_date: Option<String>,
}
