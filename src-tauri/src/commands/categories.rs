use tauri::State;

use crate::{app_state::AppState, domain::Category, storage::ChronologyRepository};

#[tauri::command]
pub async fn create_category(
    name: String,
    icon: Option<String>,
    color: Option<String>,
    state: State<'_, AppState>,
) -> Result<Category, String> {
    let category = Category::with_details(
        name,
        icon.unwrap_or_else(|| "✨".to_string()),
        color.unwrap_or_else(|| "#F59E0B".to_string()),
        None,
    )
    .map_err(|e| e.to_string())?;

    let mut service = state.service.lock().await;
    service
        .repository_mut()
        .save_category(category.clone())
        .await
        .map_err(|e| e.to_string())?;

    Ok(category)
}

#[tauri::command]
pub async fn get_categories(state: State<'_, AppState>) -> Result<Vec<Category>, String> {
    let service = state.service.lock().await;
    service
        .repository()
        .categories()
        .await
        .map_err(|e| e.to_string())
}
