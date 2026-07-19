use tauri::{Manager, State};

use crate::{app_state::AppState, domain::Entry, storage::ChronologyRepository};

#[tauri::command]
pub async fn create_entry(
    object_id: String,
    title: String,
    description: Option<String>,
    image_filenames: Option<Vec<String>>,
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut service = state.service.lock().await;
    let objects = service.repository().objects().await.map_err(|e| e.to_string())?;
    
    let object = objects.into_iter().find(|o| o.id.value().to_string() == object_id)
        .ok_or_else(|| "Object not found".to_string())?;

    // Create entry domain object
    let entry = crate::domain::Entry::new(object.id, chrono::Utc::now(), title, description)
        .map_err(|e| e.to_string())?;

    // Create photo domain objects
    let mut photos = Vec::new();
    if let Some(ref filenames) = image_filenames {
        for filename in filenames {
            let photo = crate::domain::Photo::new(entry.id, filename, filename);
            photos.push(photo);
        }
    }

    // Prepare list of successfully moved paths for potential rollback
    let mut moved_paths = Vec::new();
    let mut move_failed = false;
    let mut error_msg = String::new();

    if let Some(ref filenames) = image_filenames {
        if let Ok(app_data_dir) = app.path().app_data_dir() {
            let staging_dir = app_data_dir.join("media").join("staging");
            let originals_dir = app_data_dir.join("media").join("originals");

            for filename in filenames {
                let staging_path = staging_dir.join(filename);
                let originals_path = originals_dir.join(filename);

                if staging_path.exists() {
                    if let Err(e) = std::fs::rename(&staging_path, &originals_path) {
                        move_failed = true;
                        error_msg = format!("Failed to move file to originals: {}", e);
                        break;
                    } else {
                        moved_paths.push((staging_path, originals_path));
                    }
                } else {
                    move_failed = true;
                    error_msg = format!("Staged file does not exist: {}", filename);
                    break;
                }
            }
        }
    }

    if move_failed {
        // Rollback: move any successfully moved files back to staging
        for (staging_path, originals_path) in moved_paths {
            let _ = std::fs::rename(originals_path, staging_path);
        }
        return Err(error_msg);
    }

    // Persist entry and photos atomically in DB
    match service
        .repository_mut()
        .save_entry_with_photos(entry.clone(), photos)
        .await
    {
        Ok(_) => {
            // Success! Transaction committed, files are in originals.
            Ok(entry.id.value().to_string())
        }
        Err(db_err) => {
            // DB transaction failed. Rollback: move files back to staging
            for (staging_path, originals_path) in moved_paths {
                let _ = std::fs::rename(originals_path, staging_path);
            }
            Err(format!("Database error: {}", db_err))
        }
    }
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
    let mut delete_errors = Vec::new();
    if let Ok(app_data_dir) = app.path().app_data_dir() {
        let media_originals_dir = app_data_dir.join("media").join("originals");
        for photo in photos {
            let file_path = media_originals_dir.join(&photo.path);
            if file_path.exists() {
                if let Err(e) = std::fs::remove_file(&file_path) {
                    eprintln!("Failed to remove file {:?}: {}", file_path, e);
                    delete_errors.push(format!("Failed to delete photo {}: {}", photo.path, e));
                }
            }
        }
    }

    if !delete_errors.is_empty() {
        return Err(format!("Partial success: entry deleted from DB, but some files could not be removed: {}", delete_errors.join("; ")));
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

#[tauri::command]
pub async fn search_entries(
    query_text: Option<String>,
    category_id: Option<String>,
    object_id: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<Entry>, String> {
    let service = state.service.lock().await;
    service
        .repository()
        .search_entries(query_text, category_id, object_id, start_date, end_date)
        .await
}
