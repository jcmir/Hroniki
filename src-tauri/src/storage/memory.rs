use crate::domain::{Category, ChronicleObject, Entry, EntryId, Photo, Reminder};

use super::ChronologyRepository;

#[derive(Default)]
pub struct MemoryChronologyRepository {
    categories: Vec<Category>,
    objects: Vec<ChronicleObject>,
    entries: Vec<Entry>,
    photos: Vec<Photo>,
    reminders: Vec<Reminder>,
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

    async fn save_entry_with_photos(&mut self, entry: Entry, photos: Vec<Photo>) -> Result<(), String> {
        self.entries.push(entry);
        self.photos.extend(photos);
        Ok(())
    }

    async fn save_photo(&mut self, photo: Photo) -> Result<(), String> {
        self.photos.push(photo);
        Ok(())
    }

    async fn save_reminder(&mut self, reminder: Reminder) -> Result<(), String> {
        if let Some(pos) = self.reminders.iter().position(|r| r.id == reminder.id) {
            self.reminders[pos] = reminder;
        } else {
            self.reminders.push(reminder);
        }
        Ok(())
    }

    async fn delete_entry(&mut self, id: EntryId) -> Result<(), String> {
        self.entries.retain(|e| e.id != id);
        self.photos.retain(|p| p.entry_id != id);
        self.reminders.retain(|r| r.entry_id != id);
        Ok(())
    }

    async fn update_entry(&mut self, id: EntryId, title: String, description: Option<String>) -> Result<(), String> {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.title = title;
            entry.description = description;
            entry.updated_at = chrono::Utc::now();
        }
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

    async fn entry_photos(&self, entry_id: EntryId) -> Result<Vec<Photo>, String> {
        let entry_photos = self.photos
            .iter()
            .filter(|p| p.entry_id == entry_id)
            .cloned()
            .collect();
        Ok(entry_photos)
    }

    async fn entry_reminders(&self, entry_id: EntryId) -> Result<Vec<Reminder>, String> {
        let entry_reminders = self.reminders
            .iter()
            .filter(|r| r.entry_id == entry_id)
            .cloned()
            .collect();
        Ok(entry_reminders)
    }

    async fn reminders(&self) -> Result<Vec<Reminder>, String> {
        Ok(self.reminders.clone())
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
