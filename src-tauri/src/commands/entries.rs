use tauri::{Manager, State};

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

#[tauri::command]
pub async fn delete_entry(
    entry_id: String,
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let entry_uuid = uuid::Uuid::parse_str(&entry_id).map_err(|e| e.to_string())?;
    let id = crate::domain::EntryId::from(entry_uuid);

    let mut service = state.service.lock().await;

    // Get the photos associated with this entry before deleting it
    let photos = service.repository().entry_photos(id).await.map_err(|e| e.to_string())?;

    // Delete the entry from the database (cascade deletes photo database records)
    service.repository_mut().delete_entry(id).await.map_err(|e| e.to_string())?;

    // Physical deletion of original photo files from media originals directory
    if let Ok(app_data_dir) = app.path().app_data_dir() {
        let media_originals_dir = app_data_dir.join("media").join("originals");
        for photo in photos {
            let file_path = media_originals_dir.join(&photo.path);
            if file_path.exists() {
                let _ = std::fs::remove_file(file_path);
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn update_entry(
    entry_id: String,
    title: String,
    description: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let entry_uuid = uuid::Uuid::parse_str(&entry_id).map_err(|e| e.to_string())?;
    let id = crate::domain::EntryId::from(entry_uuid);

    let mut service = state.service.lock().await;
    service.repository_mut().update_entry(id, title, description).await.map_err(|e| e.to_string())?;
    Ok(())
}
