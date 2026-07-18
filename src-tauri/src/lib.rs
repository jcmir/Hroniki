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
    storage::{connection::create_pool, migrations::run_migrations, SqliteChronologyRepository, ChronologyRepository},
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
            std::fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");
            let db_path = app_data_dir.join("chronology.sqlite");
            if !db_path.exists() {
                std::fs::File::create(&db_path).expect("failed to create sqlite database file");
            }
            let db_url = format!("sqlite://{}", db_path.to_string_lossy().replace('\\', "/"));

            tauri::async_runtime::block_on(async move {
                let pool = create_pool(&db_url).await.expect("failed to create db pool");
                run_migrations(&pool).await.expect("failed to run migrations");

                let repository = SqliteChronologyRepository::new(pool);
                let mut service = ChronologyService::new(repository);

                // Populate default categories if empty
                let categories = service.repository().categories().await.expect("failed to get categories");
                if categories.is_empty() {
                    service.create_category("Сад").await.expect("failed to create category");
                    service.create_category("Здоровье").await.expect("failed to create category");
                    service.create_category("Авто").await.expect("failed to create category");
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
