use chrono::{DateTime, Duration, Utc};
use tauri::State;

use crate::{app_state::AppState, domain::Reminder, storage::ChronologyRepository};

#[tauri::command]
pub async fn create_reminder(
    entry_id: String,
    trigger_at: String,
    repeat_days: Option<i32>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let entry_uuid = uuid::Uuid::parse_str(&entry_id).map_err(|e| e.to_string())?;
    let entry_id = crate::domain::EntryId::from(entry_uuid);

    let parsed_trigger = DateTime::parse_from_rfc3339(&trigger_at)
        .map_err(|e| e.to_string())?
        .with_timezone(&Utc);

    let reminder = Reminder::new(entry_id, parsed_trigger, repeat_days);
    let reminder_id = reminder.id.value().to_string();

    let mut service = state.service.lock().await;
    service
        .repository_mut()
        .save_reminder(reminder)
        .await
        .map_err(|e| e.to_string())?;

    Ok(reminder_id)
}

#[tauri::command]
pub async fn get_reminders(state: State<'_, AppState>) -> Result<Vec<Reminder>, String> {
    let service = state.service.lock().await;
    service
        .repository()
        .reminders()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn complete_reminder(
    reminder_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let reminder_uuid = uuid::Uuid::parse_str(&reminder_id).map_err(|e| e.to_string())?;
    let id = crate::domain::ReminderId::from(reminder_uuid);

    let mut service = state.service.lock().await;

    // Find the reminder
    let reminders = service
        .repository()
        .reminders()
        .await
        .map_err(|e| e.to_string())?;
    let mut reminder = reminders
        .into_iter()
        .find(|r| r.id == id)
        .ok_or_else(|| "Reminder not found".to_string())?;

    reminder.status = "Completed".to_string();
    reminder.completed_at = Some(Utc::now());

    // Update status in DB
    service
        .repository_mut()
        .save_reminder(reminder.clone())
        .await
        .map_err(|e| e.to_string())?;

    // Auto-schedule next reminder if repeat days is configured
    if let Some(days) = reminder.repeat_days {
        let next_trigger = Utc::now() + Duration::days(days as i64);
        let next_reminder = Reminder::new(reminder.entry_id, next_trigger, Some(days));
        service
            .repository_mut()
            .save_reminder(next_reminder)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn snooze_reminder(
    reminder_id: String,
    days: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let reminder_uuid = uuid::Uuid::parse_str(&reminder_id).map_err(|e| e.to_string())?;
    let id = crate::domain::ReminderId::from(reminder_uuid);

    let mut service = state.service.lock().await;

    // Find the reminder
    let reminders = service
        .repository()
        .reminders()
        .await
        .map_err(|e| e.to_string())?;
    let mut reminder = reminders
        .into_iter()
        .find(|r| r.id == id)
        .ok_or_else(|| "Reminder not found".to_string())?;

    reminder.status = "Snoozed".to_string();
    reminder.trigger_at = Utc::now() + Duration::days(days as i64);

    service
        .repository_mut()
        .save_reminder(reminder)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
