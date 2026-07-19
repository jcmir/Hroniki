use crate::domain::{Category, ChronicleObject, Entry, EntryId, Photo, Reminder};

use super::{ChronologyRepository, ObjectStats};

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

    async fn search_entries(
        &self,
        query_text: Option<String>,
        category_id: Option<String>,
        object_id: Option<String>,
        start_date: Option<String>,
        end_date: Option<String>
    ) -> Result<Vec<Entry>, String> {
        let mut results = self.entries.clone();

        if let Some(ref text) = query_text {
            let text_lower = text.to_lowercase();
            results.retain(|e| {
                e.title.to_lowercase().contains(&text_lower)
                    || e.description.as_ref().map(|d| d.to_lowercase().contains(&text_lower)).unwrap_or(false)
            });
        }

        if let Some(ref cat_id_str) = category_id {
            if let Ok(cat_uuid) = uuid::Uuid::parse_str(cat_id_str) {
                let target_cat_id = crate::domain::CategoryId::from(cat_uuid);
                let valid_object_ids: std::collections::HashSet<crate::domain::ChronicleObjectId> = self.objects
                    .iter()
                    .filter(|o| o.category_id == target_cat_id)
                    .map(|o| o.id)
                    .collect();
                results.retain(|e| valid_object_ids.contains(&e.object_id));
            }
        }

        if let Some(ref obj_id_str) = object_id {
            if let Ok(obj_uuid) = uuid::Uuid::parse_str(obj_id_str) {
                let target_obj_id = crate::domain::ChronicleObjectId::from(obj_uuid);
                results.retain(|e| e.object_id == target_obj_id);
            }
        }

        if let Some(ref start) = start_date {
            if let Ok(start_dt) = chrono::DateTime::parse_from_rfc3339(start) {
                let start_utc = start_dt.with_timezone(&chrono::Utc);
                results.retain(|e| e.occurred_at >= start_utc);
            }
        }

        if let Some(ref end) = end_date {
            if let Ok(end_dt) = chrono::DateTime::parse_from_rfc3339(end) {
                let end_utc = end_dt.with_timezone(&chrono::Utc);
                results.retain(|e| e.occurred_at <= end_utc);
            }
        }

        results.sort_by(|a, b| b.occurred_at.cmp(&a.occurred_at));

        Ok(results)
    }

    async fn get_object_stats(&self, object_id: crate::domain::ChronicleObjectId) -> Result<ObjectStats, String> {
        let objects = self.objects.clone();
        let object = objects.into_iter().find(|o| o.id == object_id)
            .ok_or_else(|| "Object not found".to_string())?;

        let obj_entries: Vec<_> = self.entries.iter().filter(|e| e.object_id == object_id).cloned().collect();
        let age_days = (chrono::Utc::now() - object.created_at).num_days();

        let total_entries = obj_entries.len();
        
        let mut total_photos = 0;
        for entry in &obj_entries {
            let photos_count = self.photos.iter().filter(|p| p.entry_id == entry.id).count();
            total_photos += photos_count;
        }

        let mut sorted_entries = obj_entries.clone();
        sorted_entries.sort_by(|a, b| b.occurred_at.cmp(&a.occurred_at));

        let last_event = sorted_entries.first();
        let last_event_title = last_event.map(|e| e.title.clone());
        let last_event_date = last_event.map(|e| e.occurred_at.to_rfc3339());

        let mut next_reminder_date = None;
        let mut closest_trigger: Option<chrono::DateTime<chrono::Utc>> = None;

        for entry in &obj_entries {
            let reminders: Vec<_> = self.reminders.iter().filter(|r| r.entry_id == entry.id).cloned().collect();
            for reminder in reminders {
                if reminder.status == "Scheduled" {
                    match closest_trigger {
                        None => {
                            closest_trigger = Some(reminder.trigger_at);
                        }
                        Some(current) => {
                            if reminder.trigger_at < current {
                                closest_trigger = Some(reminder.trigger_at);
                            }
                        }
                    }
                }
            }
        }

        if let Some(trigger) = closest_trigger {
            next_reminder_date = Some(trigger.to_rfc3339());
        }

        Ok(ObjectStats {
            age_days: if age_days < 0 { 0 } else { age_days },
            total_entries,
            total_photos,
            last_event_title,
            last_event_date,
            next_reminder_date,
        })
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
