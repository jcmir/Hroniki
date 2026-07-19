use crate::app_state::AppState;
use crate::storage::ChronologyRepository;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use tauri::State;

/// Re-export of v2 reminder commands (delegated to reminder module)
#[tauri::command]
pub async fn create_reminder_v2(
    entry_id: Option<String>,
    title: String,
    body: Option<String>,
    trigger_at: String,
    recurrence: String,
    state: State<'_, AppState>,
) -> Result<crate::reminder::models::Reminder, String> {
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
pub async fn get_active_reminders_v2(
    state: State<'_, AppState>,
) -> Result<Vec<crate::reminder::models::Reminder>, String> {
    crate::reminder::commands::get_active_reminders_cmd(state).await
}

#[derive(Serialize, Deserialize)]
pub struct MemoryCenterItem {
    pub id: String,
    pub title: String,
    pub object_name: Option<String>,
    pub trigger_at: String,
    pub section: String, // "today", "on_this_day", "upcoming"
    pub years_ago: Option<i64>,
    pub days_until: Option<i64>,
}

/// Returns Memory Center items grouped into sections: today, on_this_day, upcoming.
#[tauri::command]
pub async fn get_memory_center(
    state: State<'_, AppState>,
) -> Result<Vec<MemoryCenterItem>, String> {
    let service = state.service.lock().await;
    let repo = service.repository();

    let reminders = repo.reminders().await.map_err(|e| e.to_string())?;
    let now = Utc::now();
    let today_naive = now.date_naive();

    let mut items: Vec<MemoryCenterItem> = Vec::new();

    for r in reminders {
        if r.status == "Completed" {
            continue;
        }

        let trigger = r.trigger_at;
        let trigger_date = trigger.date_naive();
        let days_until = (trigger_date - today_naive).num_days();

        let section = if days_until == 0 {
            "today"
        } else if days_until > 0 && days_until <= 30 {
            "upcoming"
        } else {
            continue;
        };

        items.push(MemoryCenterItem {
            id: r.id.value().to_string(),
            title: "Запланированное событие".to_string(),
            object_name: None,
            trigger_at: trigger.to_rfc3339(),
            section: section.to_string(),
            years_ago: None,
            days_until: if days_until >= 0 {
                Some(days_until)
            } else {
                None
            },
        });
    }

    // Section: on_this_day — historical entries from ~1 year ago (±3 day window)
    let one_year_ago = now - Duration::days(365);
    let window_start = (one_year_ago - Duration::days(3)).naive_utc();
    let window_end = (one_year_ago + Duration::days(3)).naive_utc();

    let entries = repo.entries().await.map_err(|e| e.to_string())?;
    let objects = repo.objects().await.map_err(|e| e.to_string())?;

    for entry in entries {
        let entry_naive = entry.occurred_at.naive_utc();
        if entry_naive >= window_start && entry_naive <= window_end {
            let obj_name = objects
                .iter()
                .find(|o| o.id == entry.object_id)
                .map(|o| o.name.clone());

            items.push(MemoryCenterItem {
                id: entry.id.value().to_string(),
                title: entry.title.clone(),
                object_name: obj_name,
                trigger_at: entry.occurred_at.to_rfc3339(),
                section: "on_this_day".to_string(),
                years_ago: Some(1),
                days_until: None,
            });
        }
    }

    Ok(items)
}
