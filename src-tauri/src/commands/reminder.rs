use crate::app_state::AppState;
use crate::reminder::models::Reminder;
use tauri::State;

#[tauri::command]
pub async fn create_reminder_v2(
    entry_id: Option<String>,
    title: String,
    body: Option<String>,
    trigger_at: String,
    recurrence: String,
    state: State<'_, AppState>,
) -> Result<Reminder, String> {
    crate::reminder::commands::create_reminder_cmd(
        entry_id, title, body, trigger_at, recurrence, state,
    )
    .await
}

#[tauri::command]
pub async fn cancel_reminder_v2(id: String, state: State<'_, AppState>) -> Result<(), String> {
    crate::reminder::commands::cancel_reminder_cmd(id, state).await
}

#[tauri::command]
pub async fn complete_reminder_v2(id: String, state: State<'_, AppState>) -> Result<bool, String> {
    crate::reminder::commands::complete_reminder_cmd(id, state).await
}

#[tauri::command]
pub async fn get_active_reminders_v2(state: State<'_, AppState>) -> Result<Vec<Reminder>, String> {
    crate::reminder::commands::get_active_reminders_cmd(state).await
}
