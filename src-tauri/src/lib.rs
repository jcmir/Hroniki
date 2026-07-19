pub mod app_state;
pub mod application;
pub mod commands;
pub mod domain;
pub mod storage;
pub mod identity;
pub mod security;


use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;
use crate::{
    app_state::AppState,
    application::chronology::ChronologyService,
    storage::{connection::create_pool, SqliteChronologyRepository},
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
            
            let database_dir = app_data_dir.join("database");
            let media_originals_dir = app_data_dir.join("media").join("originals");
            let media_staging_dir = app_data_dir.join("media").join("staging");
            let media_thumbnails_dir = app_data_dir.join("media").join("thumbnails");

            std::fs::create_dir_all(&database_dir).expect("failed to create database dir");
            std::fs::create_dir_all(&media_originals_dir).expect("failed to create media originals dir");
            std::fs::create_dir_all(&media_staging_dir).expect("failed to create media staging dir");
            std::fs::create_dir_all(&media_thumbnails_dir).expect("failed to create media thumbnails dir");

            let db_path = database_dir.join("chronology.sqlite");
            if !db_path.exists() {
                std::fs::File::create(&db_path).expect("failed to create sqlite database file");
            }
            let db_url = format!("sqlite://{}", db_path.to_string_lossy().replace('\\', "/"));

            tauri::async_runtime::block_on(async move {
                let pool = create_pool(&db_url).await.expect("failed to create db pool");

                let repository = SqliteChronologyRepository::new(pool.clone());
                let mut service = ChronologyService::new(repository);

                crate::application::bootstrap::initialize_application(&mut service, &pool, &app_handle)
                    .await
                    .expect("failed to initialize application");

                crate::application::bootstrap::start_reminder_scheduler(pool.clone(), app_handle.clone());

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
            commands::objects::get_object_stats,
            commands::entries::create_entry,
            commands::entries::get_entries,
            commands::entries::get_entry_photos,
            commands::entries::delete_entry,
            commands::entries::update_entry,
            commands::entries::search_entries,
            commands::media::select_images,
            commands::media::save_media,
            commands::media::get_media_path,
            commands::media::cleanup_media,
            commands::reminders::create_reminder,
            commands::reminders::get_reminders,
            commands::reminders::complete_reminder,
            commands::reminders::snooze_reminder,
            commands::pin::is_pin_configured,
            commands::pin::set_pin,
            commands::pin::verify_pin,
            commands::pin::disable_pin,
            commands::backup::export_archive,
            commands::backup::import_archive,
            commands::onboarding::is_onboarding_completed,
            commands::onboarding::complete_onboarding,
            commands::onboarding::seed_demo_data,
            commands::onboarding::get_username,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
