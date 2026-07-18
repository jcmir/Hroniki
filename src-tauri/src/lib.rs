pub mod app_state;
pub mod application;
pub mod commands;
pub mod domain;
pub mod storage;

use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;
use crate::{
    app_state::AppState,
    application::chronology::ChronologyService,
    storage::{connection::create_pool, migrations::run_migrations, SqliteChronologyRepository},
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
            
            let database_dir = app_data_dir.join("database");
            let media_originals_dir = app_data_dir.join("media").join("originals");
            let media_thumbnails_dir = app_data_dir.join("media").join("thumbnails");

            std::fs::create_dir_all(&database_dir).expect("failed to create database dir");
            std::fs::create_dir_all(&media_originals_dir).expect("failed to create media originals dir");
            std::fs::create_dir_all(&media_thumbnails_dir).expect("failed to create media thumbnails dir");

            let db_path = database_dir.join("chronology.sqlite");
            if !db_path.exists() {
                std::fs::File::create(&db_path).expect("failed to create sqlite database file");
            }
            let db_url = format!("sqlite://{}", db_path.to_string_lossy().replace('\\', "/"));

            tauri::async_runtime::block_on(async move {
                let pool = create_pool(&db_url).await.expect("failed to create db pool");
                run_migrations(&pool).await.expect("failed to run migrations");

                // Check app_metadata for default_seed_version
                let seed_check: Result<Option<(String,)>, sqlx::Error> = sqlx::query_as(
                    "SELECT value FROM app_metadata WHERE key = 'default_seed_version'"
                )
                .fetch_optional(&pool)
                .await;

                let is_seeded = match seed_check {
                    Ok(Some((val,))) => val == "1",
                    _ => false,
                };

                let repository = SqliteChronologyRepository::new(pool.clone());
                let mut service = ChronologyService::new(repository);

                if !is_seeded {
                    // Seeding default categories
                    service.create_category("Сад").await.expect("failed to create category");
                    service.create_category("Здоровье").await.expect("failed to create category");
                    service.create_category("Авто").await.expect("failed to create category");

                    sqlx::query("INSERT INTO app_metadata (key, value) VALUES ('default_seed_version', '1')")
                        .execute(&pool)
                        .await
                        .expect("failed to insert default_seed_version");
                }

                app.manage(AppState {
                    service: Arc::new(Mutex::new(service)),
                });
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::categories::create_category,
            commands::categories::get_categories,
            commands::objects::create_object,
            commands::objects::get_objects,
            commands::entries::create_entry,
            commands::entries::get_entries,
            commands::entries::get_entry_photos,
            commands::entries::delete_entry,
            commands::entries::update_entry,
            commands::media::select_images,
            commands::media::save_media,
            commands::media::get_media_path,
            commands::reminders::create_reminder,
            commands::reminders::get_reminders,
            commands::reminders::complete_reminder,
            commands::reminders::snooze_reminder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
