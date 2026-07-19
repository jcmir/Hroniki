use crate::app_state::AppState;
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::{Manager, State};

#[derive(Serialize, Deserialize)]
pub struct CrashLogEntry {
    pub filename: String,
    pub size_bytes: u64,
    pub created_at: String,
}

/// Lists all crash log files in the diagnostics directory.
#[tauri::command]
pub async fn get_crash_logs(
    _state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<Vec<CrashLogEntry>, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    let diag_dir = app_data_dir.join("diagnostics");

    if !diag_dir.exists() {
        return Ok(vec![]);
    }

    let entries = fs::read_dir(&diag_dir).map_err(|e| e.to_string())?;

    let mut logs: Vec<CrashLogEntry> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().starts_with("crash_"))
        .filter_map(|e| {
            let meta = e.metadata().ok()?;
            let filename = e.file_name().to_string_lossy().to_string();
            let created_at = meta
                .created()
                .ok()
                .and_then(|t| {
                    t.duration_since(std::time::UNIX_EPOCH).ok().map(|d| {
                        chrono::DateTime::from_timestamp(d.as_secs() as i64, 0)
                            .unwrap_or_default()
                            .to_rfc3339()
                    })
                })
                .unwrap_or_else(|| "unknown".to_string());

            Some(CrashLogEntry {
                filename,
                size_bytes: meta.len(),
                created_at,
            })
        })
        .collect();

    // Newest first
    logs.sort_by(|a, b| b.filename.cmp(&a.filename));
    Ok(logs)
}

/// Reads the content of a specific crash log file.
#[tauri::command]
pub async fn read_crash_log(
    filename: String,
    _state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    // Sanitize: only allow crash_*.log pattern, no path traversal
    if !filename.starts_with("crash_")
        || !filename.ends_with(".log")
        || filename.contains('/')
        || filename.contains('\\')
    {
        return Err("Invalid log filename".to_string());
    }

    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    let log_path = app_data_dir.join("diagnostics").join(&filename);

    fs::read_to_string(&log_path).map_err(|e| e.to_string())
}

/// Deletes all crash log files.
#[tauri::command]
pub async fn clear_crash_logs(
    _state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<u32, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    let diag_dir = app_data_dir.join("diagnostics");

    if !diag_dir.exists() {
        return Ok(0);
    }

    let entries = fs::read_dir(&diag_dir).map_err(|e| e.to_string())?;
    let mut count = 0u32;

    for entry in entries.filter_map(|e| e.ok()) {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("crash_") && name.ends_with(".log") {
            let _ = fs::remove_file(entry.path());
            count += 1;
        }
    }

    Ok(count)
}
