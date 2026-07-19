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
