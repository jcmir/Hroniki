use super::models::{RecurrenceRule, Reminder};
use crate::app_state::AppState;
use chrono::{DateTime, Utc};
use tauri::State;

#[tauri::command]
pub async fn create_reminder_cmd(
    entry_id: Option<String>,
    title: String,
    body: Option<String>,
    trigger_at: String,
    recurrence: String,
    state: State<'_, AppState>,
) -> Result<Reminder, String> {
    let dt = DateTime::parse_from_rfc3339(&trigger_at)
        .map_err(|e| e.to_string())?
        .with_timezone(&Utc);

    let rec_rule = RecurrenceRule::parse(&recurrence);

    state
        .reminder_service
        .create_reminder(entry_id, title, body, dt, rec_rule)
        .await
}

#[tauri::command]
pub async fn cancel_reminder_cmd(id: String, state: State<'_, AppState>) -> Result<(), String> {
    state.reminder_service.cancel_reminder(&id).await
}

#[tauri::command]
pub async fn complete_reminder_cmd(id: String, state: State<'_, AppState>) -> Result<bool, String> {
    state.reminder_service.complete_reminder(&id).await
}

#[tauri::command]
pub async fn get_active_reminders_cmd(state: State<'_, AppState>) -> Result<Vec<Reminder>, String> {
    state.reminder_service.get_active_reminders().await
}
