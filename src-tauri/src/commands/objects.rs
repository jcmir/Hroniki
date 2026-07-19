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
    let categories = service
        .repository()
        .categories()
        .await
        .map_err(|e| e.to_string())?;

    let category = categories
        .into_iter()
        .find(|c| c.id.value().to_string() == category_id)
        .ok_or_else(|| "Category not found".to_string())?;

    let object = service
        .create_object(&category, name, description)
        .await
        .map_err(|e| e.to_string())?;

    Ok(object.id.value().to_string())
}

#[tauri::command]
pub async fn get_objects(state: State<'_, AppState>) -> Result<Vec<ChronicleObject>, String> {
    let service = state.service.lock().await;
    service
        .repository()
        .objects()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_object_stats(
    object_id: String,
    state: State<'_, AppState>,
) -> Result<crate::storage::ObjectStats, String> {
    let service = state.service.lock().await;
    let repo = service.repository();

    let object_uuid = uuid::Uuid::parse_str(&object_id).map_err(|e| e.to_string())?;
    let obj_id = crate::domain::ChronicleObjectId::from(object_uuid);

    repo.get_object_stats(obj_id).await
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ObjectDetailsDto {
    pub object: ChronicleObject,
    pub entries_count: usize,
    pub photos_count: usize,
    pub entries: Vec<crate::domain::Entry>,
}

#[tauri::command]
pub async fn get_object_details(
    object_id: String,
    state: State<'_, AppState>,
) -> Result<ObjectDetailsDto, String> {
    let service = state.service.lock().await;
    let repo = service.repository();

    let object_uuid = uuid::Uuid::parse_str(&object_id).map_err(|e| e.to_string())?;
    let obj_id = crate::domain::ChronicleObjectId::from(object_uuid);

    let objects = repo.objects().await.map_err(|e| e.to_string())?;
    let obj = objects
        .into_iter()
        .find(|o| o.id == obj_id)
        .ok_or_else(|| "Object not found".to_string())?;

    let all_entries = repo.entries().await.map_err(|e| e.to_string())?;
    let object_entries: Vec<crate::domain::Entry> = all_entries
        .into_iter()
        .filter(|e| e.object_id == obj_id)
        .collect();

    let mut total_photos = 0;
    for entry in &object_entries {
        let photos = repo.entry_photos(entry.id).await.unwrap_or_default();
        total_photos += photos.len();
    }

    let entries_count = object_entries.len();

    Ok(ObjectDetailsDto {
        object: obj,
        entries_count,
        photos_count: total_photos,
        entries: object_entries,
    })
}
