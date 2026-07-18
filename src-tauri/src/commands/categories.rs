use tauri::State;

use crate::{app_state::AppState, domain::Category, storage::ChronologyRepository};

#[tauri::command]
pub async fn create_category(
    name: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut service = state.service.lock().await;

    let category = service
        .create_category(name)
        .await
        .map_err(|e| e.to_string())?;

    Ok(category.id.value().to_string())
}

#[tauri::command]
pub async fn get_categories(
    state: State<'_, AppState>,
) -> Result<Vec<Category>, String> {
    let service = state.service.lock().await;
    service.repository().categories().await.map_err(|e| e.to_string())
}
