use crate::{
    app_state::AppState,
    application::{chronology::ChronologyService, security},
    storage::{connection::create_pool, migrations::run_migrations, SqliteChronologyRepository},
};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use tauri::{Manager, State};

#[derive(Serialize, Deserialize, Debug)]
pub struct ArchiveManifest {
    pub app_name: String,
    pub version: String,
    pub schema_version: u32,
    pub encryption_version: String,
    pub exported_at: String,
}

#[tauri::command]
pub async fn export_archive(
    password: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    if password.is_empty() {
        return Err("Password cannot be empty".to_string());
    }

    // 1. Show save dialog to let user select save path
    let target_path = match pick_export_path()? {
        Some(path) => path,
        None => return Ok("Export cancelled".to_string()),
    };

    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let database_dir = app_data_dir.join("database");
    let temp_db_path = database_dir.join("temp_backup.sqlite");

    if temp_db_path.exists() {
        let _ = std::fs::remove_file(&temp_db_path);
    }

    // 2. Safely snapshot database using VACUUM INTO to prevent locks/corruption
    let service = state.service.lock().await;
    let pool = service.repository().pool();

    sqlx::query(&format!(
        "VACUUM INTO '{}'",
        temp_db_path.to_string_lossy().replace('\\', "/")
    ))
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    // 3. Create ZIP archive in memory
    let mut zip_buf = Vec::new();
    {
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_buf));
        let options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        // Add chronology.sqlite to ZIP
        zip.start_file("chronology.sqlite", options)
            .map_err(|e| e.to_string())?;
        let db_bytes = std::fs::read(&temp_db_path).map_err(|e| e.to_string())?;
        zip.write_all(&db_bytes).map_err(|e| e.to_string())?;

        // Add manifest.json for Archive Format v2
        let manifest = ArchiveManifest {
            app_name: "ХРОНИКИ".to_string(),
            version: "1.0.0".to_string(),
            schema_version: 6, // current version matching migration 0006
            encryption_version: "AES-256-GCM".to_string(),
            exported_at: chrono::Utc::now().to_rfc3339(),
        };
        let manifest_json = serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?;
        zip.start_file("manifest.json", options)
            .map_err(|e| e.to_string())?;
        zip.write_all(manifest_json.as_bytes())
            .map_err(|e| e.to_string())?;

        // Add media files to ZIP
        let originals_dir = app_data_dir.join("media").join("originals");
        if originals_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(originals_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(filename_os) = path.file_name() {
                            let filename = filename_os.to_string_lossy();
                            zip.start_file(format!("media/{}", filename), options)
                                .map_err(|e| e.to_string())?;
                            let file_bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
                            zip.write_all(&file_bytes).map_err(|e| e.to_string())?;
                        }
                    }
                }
            }
        }
        zip.finish().map_err(|e| e.to_string())?;
    }

    // Clean up temp database snapshot
    let _ = std::fs::remove_file(&temp_db_path);

    // 4. Encrypt ZIP bytes using PBKDF2 + AES-GCM
    let encrypted_bytes = security::encrypt_data(&zip_buf, &password)?;

    // 5. Write to destination file
    std::fs::write(&target_path, encrypted_bytes).map_err(|e| e.to_string())?;

    Ok("Export completed successfully".to_string())
}

#[tauri::command]
pub async fn import_archive(
    password: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    if password.is_empty() {
        return Err("Password cannot be empty".to_string());
    }

    // 1. Select backup file
    let backup_path = match pick_import_path()? {
        Some(path) => path,
        None => return Err("No file selected".to_string()),
    };

    let encrypted_bytes = std::fs::read(&backup_path).map_err(|e| e.to_string())?;

    // 2. Decrypt zip archive bytes
    let zip_bytes = security::decrypt_data(&encrypted_bytes, &password)?;

    // 3. Close the DB pool connection so we can safely overwrite chronology.sqlite
    let mut service = state.service.lock().await;
    let pool = service.repository().pool();
    pool.close().await;

    // 4. Extract zip archive contents
    let reader = std::io::Cursor::new(zip_bytes);
    let mut archive = zip::ZipArchive::new(reader).map_err(|e| e.to_string())?;

    // Verify manifest if it exists (Archive Format v2)
    let mut manifest: Option<ArchiveManifest> = None;
    if let Ok(mut manifest_file) = archive.by_name("manifest.json") {
        let mut manifest_str = String::new();
        if manifest_file.read_to_string(&mut manifest_str).is_ok() {
            if let Ok(parsed) = serde_json::from_str::<ArchiveManifest>(&manifest_str) {
                manifest = Some(parsed);
            }
        }
    }

    // Reject import if the archive format version is newer than current app database schema
    if let Some(m) = manifest {
        if m.schema_version > 6 {
            // Re-establish original connection pool so the app does not crash or freeze
            let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
            let db_path = app_data_dir.join("database").join("chronology.sqlite");
            let db_url = format!("sqlite://{}", db_path.to_string_lossy().replace('\\', "/"));
            let restored_pool = create_pool(&db_url).await.map_err(|e| e.to_string())?;
            let repository = SqliteChronologyRepository::new(restored_pool);
            *service = ChronologyService::new(repository);

            return Err(format!(
                "Невозможно импортировать архив. Он был экспортирован из более новой версии приложения (версия схемы {}). Пожалуйста, обновите приложение ХРОНИКИ до актуальной версии.",
                m.schema_version
            ));
        }
    }

    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_data_dir.join("database").join("chronology.sqlite");
    let originals_dir = app_data_dir.join("media").join("originals");

    // Extract files
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if outpath.to_string_lossy() == "chronology.sqlite" {
            // Write new database file
            let mut outfile = std::fs::File::create(&db_path).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        } else if outpath.starts_with("media/") {
            // Write media files
            if let Some(filename) = outpath.file_name() {
                let dest_file_path = originals_dir.join(filename);
                let mut outfile =
                    std::fs::File::create(&dest_file_path).map_err(|e| e.to_string())?;
                std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
            }
        }
    }

    // 5. Reinitialize the database pool connection & run migrations
    let db_url = format!("sqlite://{}", db_path.to_string_lossy().replace('\\', "/"));
    let restored_pool = create_pool(&db_url).await.map_err(|e| e.to_string())?;
    run_migrations(&restored_pool)
        .await
        .map_err(|e| e.to_string())?;

    // Update the AppState service with the new pool connection
    let repository = SqliteChronologyRepository::new(restored_pool);
    *service = ChronologyService::new(repository);

    Ok(())
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn pick_export_path() -> Result<Option<std::path::PathBuf>, String> {
    let file_path = rfd::FileDialog::new()
        .add_filter("Hroniki Backup (*.hroniki)", &["hroniki"])
        .set_file_name("hroniki_backup.hroniki")
        .save_file();
    Ok(file_path)
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn pick_export_path() -> Result<Option<std::path::PathBuf>, String> {
    Err("Native mobile file dialog is not connected yet".to_string())
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn pick_import_path() -> Result<Option<std::path::PathBuf>, String> {
    let file_path = rfd::FileDialog::new()
        .add_filter("Hroniki Backup (*.hroniki)", &["hroniki"])
        .pick_file();
    Ok(file_path)
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn pick_import_path() -> Result<Option<std::path::PathBuf>, String> {
    Err("Native mobile file dialog is not connected yet".to_string())
}
