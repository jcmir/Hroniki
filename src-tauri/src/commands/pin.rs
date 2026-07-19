use crate::{app_state::AppState, application::security};
use tauri::State;

#[tauri::command]
pub async fn is_pin_configured(state: State<'_, AppState>) -> Result<bool, String> {
    let service = state.service.lock().await;
    let pool = service.repository().pool();

    let row: Option<(String,)> =
        sqlx::query_as("SELECT value FROM app_metadata WHERE key = 'pin_hash'")
            .fetch_optional(pool)
            .await
            .map_err(|e| e.to_string())?;

    Ok(row.is_some())
}

#[tauri::command]
pub async fn set_pin(pin: String, state: State<'_, AppState>) -> Result<(), String> {
    if pin.len() < 4 {
        return Err("PIN must be at least 4 digits".to_string());
    }

    let service = state.service.lock().await;
    let pool = service.repository().pool();

    let salt_bytes = security::generate_salt();
    let salt_hex = security::to_hex(&salt_bytes);
    let hash_hex = security::hash_pin(&pin, &salt_bytes);

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    sqlx::query("INSERT OR REPLACE INTO app_metadata (key, value) VALUES ('pin_hash', ?)")
        .bind(hash_hex)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query("INSERT OR REPLACE INTO app_metadata (key, value) VALUES ('pin_salt', ?)")
        .bind(salt_hex)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn verify_pin(pin: String, state: State<'_, AppState>) -> Result<bool, String> {
    let service = state.service.lock().await;
    let pool = service.repository().pool();

    let hash_row: Option<(String,)> =
        sqlx::query_as("SELECT value FROM app_metadata WHERE key = 'pin_hash'")
            .fetch_optional(pool)
            .await
            .map_err(|e| e.to_string())?;

    let salt_row: Option<(String,)> =
        sqlx::query_as("SELECT value FROM app_metadata WHERE key = 'pin_salt'")
            .fetch_optional(pool)
            .await
            .map_err(|e| e.to_string())?;

    if let (Some((hash_hex,)), Some((salt_hex,))) = (hash_row, salt_row) {
        let salt_bytes = security::from_hex(&salt_hex)?;
        let computed_hash = security::hash_pin(&pin, &salt_bytes);
        Ok(computed_hash == hash_hex)
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn disable_pin(state: State<'_, AppState>) -> Result<(), String> {
    let service = state.service.lock().await;
    let pool = service.repository().pool();

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    sqlx::query("DELETE FROM app_metadata WHERE key = 'pin_hash'")
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query("DELETE FROM app_metadata WHERE key = 'pin_salt'")
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}
