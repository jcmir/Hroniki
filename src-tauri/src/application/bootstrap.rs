use crate::application::chronology::ChronologyService;
use crate::storage::{migrations::run_migrations, ChronologyRepository};
use crate::commands::media::cleanup_staging;
use sqlx::SqlitePool;

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
