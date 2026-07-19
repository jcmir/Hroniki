use crate::{
    app_state::AppState,
    domain::{Category, ChronicleObject, Entry},
    storage::ChronologyRepository,
};
use tauri::State;

#[tauri::command]
pub async fn seed_demo_dataset(state: State<'_, AppState>) -> Result<bool, String> {
    let mut service = state.service.lock().await;
    let repo = service.repository_mut();

    let existing_categories = repo.categories().await.unwrap_or_default();
    if !existing_categories.is_empty() {
        return Ok(false); // Demo already seeded or user created data
    }

    // Seed default categories
    let cat_auto =
        Category::with_details("Автомобили", "🚗", "#3B82F6", Some("vehicle".to_string()))
            .map_err(|e| e.to_string())?;
    let cat_home = Category::with_details("Дом и Дача", "🏡", "#10B981", Some("home".to_string()))
        .map_err(|e| e.to_string())?;
    let cat_travel =
        Category::with_details("Путешествия", "✈️", "#EC4899", Some("travel".to_string()))
            .map_err(|e| e.to_string())?;

    repo.save_category(cat_auto.clone())
        .await
        .map_err(|e| e.to_string())?;
    repo.save_category(cat_home.clone())
        .await
        .map_err(|e| e.to_string())?;
    repo.save_category(cat_travel.clone())
        .await
        .map_err(|e| e.to_string())?;

    // Seed default objects & entries
    let obj_bmw = ChronicleObject::new(cat_auto.id, "BMW X5", Some("Семейная машина".to_string()))
        .map_err(|e| e.to_string())?;
    repo.save_object(obj_bmw.clone())
        .await
        .map_err(|e| e.to_string())?;

    let entry_bmw = Entry::new(
        obj_bmw.id,
        chrono::Utc::now(),
        "Плановое ТО и замена масла".to_string(),
        Some("Замена масла 5w30 и фильтров".to_string()),
    )
    .map_err(|e| e.to_string())?;
    repo.save_entry_with_photos(entry_bmw, vec![])
        .await
        .map_err(|e| e.to_string())?;

    let obj_home = ChronicleObject::new(
        cat_home.id,
        "Дом в Завидово",
        Some("Дача и сад".to_string()),
    )
    .map_err(|e| e.to_string())?;
    repo.save_object(obj_home.clone())
        .await
        .map_err(|e| e.to_string())?;

    let entry_home = Entry::new(
        obj_home.id,
        chrono::Utc::now(),
        "Обработка сада от вредителей".to_string(),
        Some("Обработаны яблони и груши".to_string()),
    )
    .map_err(|e| e.to_string())?;
    repo.save_entry_with_photos(entry_home, vec![])
        .await
        .map_err(|e| e.to_string())?;

    Ok(true)
}
