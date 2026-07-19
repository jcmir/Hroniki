use std::io::Write;
use tauri::{State, Manager};
use crate::{app_state::AppState, application::security};

#[tauri::command]
pub async fn export_archive(password: String, state: State<'_, AppState>, app: tauri::AppHandle) -> Result<String, String> {
    if password.is_empty() {
        return Err("Password cannot be empty".to_string());
    }

    // 1. Show save dialog to let user select save path
    let file_path = rfd::FileDialog::new()
        .add_filter("Hroniki Backup (*.hroniki)", &["hroniki"])
        .set_file_name("hroniki_backup.hroniki")
        .save_file();

    let target_path = match file_path {
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

    sqlx::query(&format!("VACUUM INTO '{}'", temp_db_path.to_string_lossy().replace('\\', "/")))
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
        zip.start_file("chronology.sqlite", options).map_err(|e| e.to_string())?;
        let db_bytes = std::fs::read(&temp_db_path).map_err(|e| e.to_string())?;
        zip.write_all(&db_bytes).map_err(|e| e.to_string())?;

        // Add media files to ZIP
        let originals_dir = app_data_dir.join("media").join("originals");
        if originals_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(originals_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(filename_os) = path.file_name() {
                            let filename = filename_os.to_string_lossy();
                            zip.start_file(format!("media/{}", filename), options).map_err(|e| e.to_string())?;
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
pub async fn import_archive(password: String, state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    if password.is_empty() {
        return Err("Password cannot be empty".to_string());
    }

    // 1. Select backup file
    let file_path = rfd::FileDialog::new()
        .add_filter("Hroniki Backup (*.hroniki)", &["hroniki"])
        .pick_file();

    let backup_path = match file_path {
        Some(path) => path,
        None => return Err("No file selected".to_string()),
    };

    let encrypted_bytes = std::fs::read(&backup_path).map_err(|e| e.to_string())?;

    // 2. Decrypt zip archive bytes
    let zip_bytes = security::decrypt_data(&encrypted_bytes, &password)?;

    // 3. Close the DB pool connection so we can safely overwrite chronology.sqlite
    let service = state.service.lock().await;
    let pool = service.repository().pool();
    pool.close().await;

    // 4. Extract zip archive contents
    let reader = std::io::Cursor::new(zip_bytes);
    let mut archive = zip::ZipArchive::new(reader).map_err(|e| e.to_string())?;

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
                let mut outfile = std::fs::File::create(&dest_file_path).map_err(|e| e.to_string())?;
                std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}
