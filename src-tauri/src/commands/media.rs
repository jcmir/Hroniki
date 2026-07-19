use tauri::Manager;

#[tauri::command]
pub async fn select_images() -> Result<Option<Vec<String>>, String> {
    let files = tauri::async_runtime::spawn_blocking(move || {
        rfd::FileDialog::new()
            .add_filter("Images", &["jpg", "jpeg", "png", "webp", "gif"])
            .pick_files()
    })
    .await
    .map_err(|e| e.to_string())?;

    match files {
        Some(paths) => {
            let string_paths = paths
                .into_iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect();
            Ok(Some(string_paths))
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn save_media(
    app: tauri::AppHandle,
    source_path: String,
) -> Result<String, String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let media_staging_dir = app_data_dir.join("media").join("staging");

    let source = std::path::Path::new(&source_path);
    if !source.exists() {
        return Err("Source file does not exist".to_string());
    }

    let extension = source
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("jpg");

    let filename = format!("{}.{}", uuid::Uuid::new_v4(), extension);
    let target_path = media_staging_dir.join(&filename);

    std::fs::copy(source, &target_path).map_err(|e| e.to_string())?;

    Ok(filename)
}

#[tauri::command]
pub fn get_media_path(
    app: tauri::AppHandle,
    filename: String,
) -> Result<String, String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let file_path = app_data_dir.join("media").join("originals").join(filename);
    Ok(file_path.to_string_lossy().into_owned())
}

pub fn cleanup_staging(app: &tauri::AppHandle) -> Result<(), String> {
    if let Ok(app_data_dir) = app.path().app_data_dir() {
        let staging_dir = app_data_dir.join("media").join("staging");
        if staging_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(staging_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let _ = std::fs::remove_file(path);
                    }
                }
            }
        }
    }
    Ok(())
}
