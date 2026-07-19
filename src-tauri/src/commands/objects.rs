use tauri::State;

use crate::{app_state::AppState, domain::ChronicleObject, storage::ChronologyRepository};

#[tauri::command]
pub async fn create_object(
    category_id: String,
    name: String,
    description: Option<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut service = state.service.lock().await;
    let categories = service.repository().categories().await.map_err(|e| e.to_string())?;
    
    let category = categories.into_iter().find(|c| c.id.value().to_string() == category_id)
        .ok_or_else(|| "Category not found".to_string())?;

    let object = service
        .create_object(&category, name, description)
        .await
        .map_err(|e| e.to_string())?;

    Ok(object.id.value().to_string())
}

#[tauri::command]
pub async fn get_objects(
    state: State<'_, AppState>,
) -> Result<Vec<ChronicleObject>, String> {
    let service = state.service.lock().await;
    service.repository().objects().await.map_err(|e| e.to_string())
}

#[derive(serde::Serialize)]
pub struct ObjectStats {
    pub age_days: i64,
    pub total_entries: usize,
    pub total_photos: usize,
    pub last_event_title: Option<String>,
    pub last_event_date: Option<String>,
    pub next_reminder_date: Option<String>,
}

#[tauri::command]
pub async fn get_object_stats(
    object_id: String,
    state: State<'_, AppState>,
) -> Result<ObjectStats, String> {
    let service = state.service.lock().await;
    let repo = service.repository();

    let object_uuid = uuid::Uuid::parse_str(&object_id).map_err(|e| e.to_string())?;
    let obj_id = crate::domain::ChronicleObjectId::from(object_uuid);

    // 1. Find object to get created_at
    let objects = repo.objects().await.map_err(|e| e.to_string())?;
    let object = objects.into_iter().find(|o| o.id == obj_id)
        .ok_or_else(|| "Object not found".to_string())?;

    let age_days = (chrono::Utc::now() - object.created_at).num_days();

    // 2. Fetch all entries and filter by this object
    let all_entries = repo.entries().await.map_err(|e| e.to_string())?;
    let mut obj_entries: Vec<_> = all_entries.into_iter().filter(|e| e.object_id == obj_id).collect();

    // Sort by occurred_at desc to find the last event
    obj_entries.sort_by(|a, b| b.occurred_at.cmp(&a.occurred_at));

    let total_entries = obj_entries.len();

    let last_event = obj_entries.first();
    let last_event_title = last_event.map(|e| e.title.clone());
    let last_event_date = last_event.map(|e| e.occurred_at.to_rfc3339());

    // 3. Count total photos
    let mut total_photos = 0;
    for entry in &obj_entries {
        if let Ok(photos) = repo.entry_photos(entry.id).await {
            total_photos += photos.len();
        }
    }

    // 4. Find next scheduled reminder
    let mut next_reminder_date = None;
    let mut closest_trigger: Option<chrono::DateTime<chrono::Utc>> = None;

    for entry in &obj_entries {
        if let Ok(reminders) = repo.entry_reminders(entry.id).await {
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
