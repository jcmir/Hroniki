use tauri::State;

use crate::{app_state::AppState, domain::Entry, storage::ChronologyRepository};

#[tauri::command]
pub async fn create_entry(
    object_id: String,
    title: String,
    description: Option<String>,
    image_filenames: Option<Vec<String>>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut service = state.service.lock().await;
    let objects = service.repository().objects().await.map_err(|e| e.to_string())?;
    
    let object = objects.into_iter().find(|o| o.id.value().to_string() == object_id)
        .ok_or_else(|| "Object not found".to_string())?;

    let entry = service
        .create_entry(&object, title, description)
        .await
        .map_err(|e| e.to_string())?;

    // Persist associated photos
    if let Some(filenames) = image_filenames {
        for filename in filenames {
            let photo = crate::domain::Photo::new(entry.id, &filename, &filename);
            service
                .repository_mut()
                .save_photo(photo)
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(entry.id.value().to_string())
}

#[tauri::command]
pub async fn get_entries(
    state: State<'_, AppState>,
) -> Result<Vec<Entry>, String> {
    let service = state.service.lock().await;
    service.repository().entries().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_entry_photos(
    entry_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<crate::domain::Photo>, String> {
    let service = state.service.lock().await;
    let entry_uuid = uuid::Uuid::parse_str(&entry_id).map_err(|e| e.to_string())?;
    let id = crate::domain::EntryId::from(entry_uuid);
    service.repository().entry_photos(id).await.map_err(|e| e.to_string())
}
