use tauri::State;
use crate::app_state::AppState;
use chrono::Utc;

#[tauri::command]
pub async fn is_onboarding_completed(state: State<'_, AppState>) -> Result<bool, String> {
    let service = state.service.lock().await;
    let pool = service.repository().pool();

    let row: Option<(String,)> = sqlx::query_as("SELECT value FROM app_metadata WHERE key = 'onboarding_completed'")
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

    if let Some((val,)) = row {
        Ok(val == "true")
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn get_username(state: State<'_, AppState>) -> Result<String, String> {
    let service = state.service.lock().await;
    let pool = service.repository().pool();

    let row: Option<(String,)> = sqlx::query_as("SELECT value FROM app_metadata WHERE key = 'username'")
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

    if let Some((val,)) = row {
        Ok(val)
    } else {
        Ok("Пользователь".to_string())
    }
}

#[tauri::command]
pub async fn complete_onboarding_impl(username: &str, pool: &sqlx::SqlitePool) -> Result<(), String> {
    if username.trim().is_empty() {
        return Err("Username cannot be empty".to_string());
    }

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    sqlx::query("INSERT OR REPLACE INTO app_metadata (key, value) VALUES ('onboarding_completed', 'true')")
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query("INSERT OR REPLACE INTO app_metadata (key, value) VALUES ('username', ?)")
        .bind(username.trim())
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn complete_onboarding(username: String, state: State<'_, AppState>) -> Result<(), String> {
    let service = state.service.lock().await;
    let pool = service.repository().pool();
    complete_onboarding_impl(&username, pool).await
}

pub async fn seed_demo_data_impl(pool: &sqlx::SqlitePool) -> Result<(), String> {
    // Check if onboarding is already completed to prevent accidental data loss
    let row: Option<(String,)> = sqlx::query_as("SELECT value FROM app_metadata WHERE key = 'onboarding_completed'")
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

    if let Some((val,)) = row {
        if val == "true" {
            return Err("Demo seeding is only allowed during onboarding".to_string());
        }
    }

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    // 1. Clear old data to prevent key conflicts
    sqlx::query("DELETE FROM reminders").execute(&mut *tx).await.map_err(|e| e.to_string())?;
    sqlx::query("DELETE FROM photos").execute(&mut *tx).await.map_err(|e| e.to_string())?;
    sqlx::query("DELETE FROM entries").execute(&mut *tx).await.map_err(|e| e.to_string())?;
    sqlx::query("DELETE FROM objects").execute(&mut *tx).await.map_err(|e| e.to_string())?;
    sqlx::query("DELETE FROM categories").execute(&mut *tx).await.map_err(|e| e.to_string())?;

    // 2. Insert Categories
    sqlx::query("INSERT INTO categories (id, name, created_at) VALUES ('cat-garden', 'Сад', ?)")
        .bind(Utc::now().to_rfc3339())
        .execute(&mut *tx).await.map_err(|e| e.to_string())?;
    sqlx::query("INSERT INTO categories (id, name, created_at) VALUES ('cat-home', 'Дом', ?)")
        .bind(Utc::now().to_rfc3339())
        .execute(&mut *tx).await.map_err(|e| e.to_string())?;
    sqlx::query("INSERT INTO categories (id, name, created_at) VALUES ('cat-auto', 'Имущество', ?)")
        .bind(Utc::now().to_rfc3339())
        .execute(&mut *tx).await.map_err(|e| e.to_string())?;

    // 3. Insert Objects
    sqlx::query("INSERT INTO objects (id, category_id, name, description, created_at) VALUES ('obj-apple', 'cat-garden', 'Любимая Яблоня', 'Сорт Антоновка, посажен в центре сада', ?)")
        .bind((Utc::now() - chrono::Duration::days(365)).to_rfc3339())
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query("INSERT INTO objects (id, category_id, name, description, created_at) VALUES ('obj-thermostat', 'cat-home', 'Дом и Дача', 'Система умного климата на веранде и в гостиной', ?)")
        .bind((Utc::now() - chrono::Duration::days(40)).to_rfc3339())
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query("INSERT INTO objects (id, category_id, name, description, created_at) VALUES ('obj-car', 'cat-auto', 'Семейный Автомобиль', 'Кроссовер, ТО каждые 10 000 км или раз в год', ?)")
        .bind((Utc::now() - chrono::Duration::days(180)).to_rfc3339())
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    // 4. Insert Entries
    // Garden
    sqlx::query("INSERT INTO entries (id, object_id, occurred_at, title, description, created_at, updated_at) VALUES ('ent-g1', 'obj-apple', ?, 'Посадка саженца Антоновки', 'Посадили молодой саженец с детьми. Обильно полили, добавили торф и укрепили колышком.', ?, ?)")
        .bind((Utc::now() - chrono::Duration::days(365)).to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query("INSERT INTO entries (id, object_id, occurred_at, title, description, created_at, updated_at) VALUES ('ent-g2', 'obj-apple', ?, 'Весенняя побелка ствола', 'Побелили ствол для защиты от грызунов и весенних ожогов. Добавили медный купорос.', ?, ?)")
        .bind((Utc::now() - chrono::Duration::days(60)).to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query("INSERT INTO entries (id, object_id, occurred_at, title, description, created_at, updated_at) VALUES ('ent-g3', 'obj-apple', ?, 'Первый урожай яблок!', 'Собрали первые 5 спелых яблок! Очень сочные, с легкой кислинкой. Растет отлично.', ?, ?)")
        .bind((Utc::now() - chrono::Duration::days(1)).to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    // Home
    sqlx::query("INSERT INTO entries (id, object_id, occurred_at, title, description, created_at, updated_at) VALUES ('ent-h1', 'obj-thermostat', ?, 'Установка термостата', 'Смонтировали беспроводной умный термостат. Настроили автоматическое расписание: +21°C днем и +18°C ночью.', ?, ?)")
        .bind((Utc::now() - chrono::Duration::days(3)).to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    // Auto
    sqlx::query("INSERT INTO entries (id, object_id, occurred_at, title, description, created_at, updated_at) VALUES ('ent-a1', 'obj-car', ?, 'Сезонное ТО и замена масла', 'Заменили моторное масло (5W-30), масляный, воздушный и салонный фильтры. Износ тормозных колодок 35%.', ?, ?)")
        .bind((Utc::now() - chrono::Duration::days(10)).to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    // 5. Insert Photos
    sqlx::query("INSERT INTO photos (id, entry_id, path, thumbnail, created_at) VALUES ('photo-demo', 'ent-g3', 'demo_apple.jpg', 'demo_apple.jpg', ?)")
        .bind(Utc::now().to_rfc3339())
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    // 6. Insert Reminder
    let reminder_trigger = Utc::now() + chrono::Duration::days(14);
    sqlx::query("INSERT INTO reminders (id, entry_id, trigger_at, status, repeat_days) VALUES ('rem-demo', 'ent-a1', ?, 'Scheduled', 180)")
        .bind(reminder_trigger.to_rfc3339())
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn seed_demo_data(state: State<'_, AppState>) -> Result<(), String> {
    let service = state.service.lock().await;
    let pool = service.repository().pool();
    seed_demo_data_impl(pool).await
}

#[cfg(test)]
mod tests {

    use crate::storage::connection::create_pool;
    use crate::storage::migrations::run_migrations;

    #[tokio::test]
    async fn test_onboarding_db_flow() {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        run_migrations(&pool).await.unwrap();

        // 1. Verify default (empty)
        let row: Option<(String,)> = sqlx::query_as("SELECT value FROM app_metadata WHERE key = 'onboarding_completed'")
            .fetch_optional(&pool)
            .await
            .unwrap();
        assert!(row.is_none());

        // 2. Set username and complete onboarding
        sqlx::query("INSERT OR REPLACE INTO app_metadata (key, value) VALUES ('onboarding_completed', 'true')")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT OR REPLACE INTO app_metadata (key, value) VALUES ('username', 'Александр')")
            .execute(&pool)
            .await
            .unwrap();

        // 3. Verify onboarding completed
        let row_completed: (String,) = sqlx::query_as("SELECT value FROM app_metadata WHERE key = 'onboarding_completed'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(row_completed.0, "true");

        // 4. Verify username stored
        let name_row: (String,) = sqlx::query_as("SELECT value FROM app_metadata WHERE key = 'username'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(name_row.0, "Александр");
    }

    #[tokio::test]
    async fn test_onboarding_demo_seed_protection() {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        run_migrations(&pool).await.unwrap();

        // 1. First seed should succeed (onboarding is false)
        super::seed_demo_data_impl(&pool).await.unwrap();

        // 2. Mark onboarding as completed
        super::complete_onboarding_impl("Александр", &pool).await.unwrap();

        // 3. Second seed should fail with error
        let result = super::seed_demo_data_impl(&pool).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Demo seeding is only allowed during onboarding");
    }
}
