use tauri::State;

use crate::app_state::AppState;


#[tauri::command]
pub async fn create_category(
    name: String,
    state: State<'_, AppState>,
) -> Result<String, String> {

    let mut service =
        state.service.lock().await;


    let category =
        service
            .create_category(name)
            .await
            .map_err(|e| e.to_string())?;


    Ok(
        category.id.value()
            .to_string()
    )
}
