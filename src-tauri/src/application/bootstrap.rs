use crate::application::chronology::ChronologyService;
use crate::storage::{migrations::run_migrations, ChronologyRepository};
use crate::commands::media::cleanup_staging;
use sqlx::SqlitePool;
use tauri::Manager;

pub async fn initialize_application<R>(
    service: &mut ChronologyService<R>,
    pool: &SqlitePool,
    app: &tauri::AppHandle,
) -> Result<(), String>
where
    R: ChronologyRepository,
{
    // Run database migrations
    run_migrations(pool).await.map_err(|e| e.to_string())?;

    // Clean up media staging directory from leftovers
    let _ = cleanup_staging(app);

    // Clean up orphan media files in originals
    let _ = cleanup_orphan_media(pool, app).await;

    // Check app_metadata for default_seed_version
    let seed_check: Result<Option<(String,)>, sqlx::Error> = sqlx::query_as(
        "SELECT value FROM app_metadata WHERE key = 'default_seed_version'"
    )
    .fetch_optional(pool)
    .await;

    let is_seeded = match seed_check {
        Ok(Some((val,))) => val == "1",
        _ => false,
    };

    if !is_seeded {
        // Seed default categories
        service.create_category("Сад").await.map_err(|e| e.to_string())?;
        service.create_category("Здоровье").await.map_err(|e| e.to_string())?;
        service.create_category("Авто").await.map_err(|e| e.to_string())?;

        sqlx::query("INSERT INTO app_metadata (key, value) VALUES ('default_seed_version', '1')")
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub async fn cleanup_orphan_media(
    pool: &SqlitePool,
    app: &tauri::AppHandle,
) -> Result<(), String> {
    // 1. Fetch all photo paths from the database
    let rows: Result<Vec<(String,)>, sqlx::Error> = sqlx::query_as("SELECT path FROM photos")
        .fetch_all(pool)
        .await;

    let db_photo_filenames: std::collections::HashSet<String> = match rows {
        Ok(paths) => paths.into_iter().map(|(p,)| p).collect(),
        Err(e) => return Err(e.to_string()),
    };

    // 2. Scan media/originals/
    if let Ok(app_data_dir) = app.path().app_data_dir() {
        let originals_dir = app_data_dir.join("media").join("originals");
        if originals_dir.exists() {
            if let Ok(dir_entries) = std::fs::read_dir(originals_dir) {
                for dir_entry in dir_entries.flatten() {
                    let path = dir_entry.path();
                    if path.is_file() {
                        if let Some(filename_os) = path.file_name() {
                            let filename = filename_os.to_string_lossy().into_owned();
                            if !db_photo_filenames.contains(&filename) {
                                // Delete orphan file
                                let _ = std::fs::remove_file(&path);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn start_reminder_scheduler(pool: SqlitePool, app_handle: tauri::AppHandle) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
        loop {
            interval.tick().await;
            let now = chrono::Utc::now();

            let reminders_res: Result<Vec<(String, String, String)>, sqlx::Error> = sqlx::query_as(
                r#"
                SELECT r.id, r.entry_id, e.title 
                FROM reminders r
                JOIN entries e ON r.entry_id = e.id
                WHERE r.status = 'Scheduled' AND r.trigger_at <= ?
                "#
            )
            .bind(now.to_rfc3339())
            .fetch_all(&pool)
            .await;

            if let Ok(reminders) = reminders_res {
                for (reminder_id, _entry_id, title) in reminders {
                    // Send notification
                    use tauri_plugin_notification::NotificationExt;
                    let _ = app_handle.notification()
                        .builder()
                        .title("ХРОНИКИ — Напоминание")
                        .body(format!("Пора вернуться к истории: {}", title))
                        .show();

                    // Update status in DB
                    let _ = sqlx::query("UPDATE reminders SET status = 'Triggered' WHERE id = ?")
                        .bind(reminder_id)
                        .execute(&pool)
                        .await;
                }
            }
        }
    });
}
