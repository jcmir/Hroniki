use crate::app_state::AppState;
use crate::search::SearchQuery;
use tauri::State;

#[derive(serde::Serialize)]
pub struct SearchResultDto {
    pub entry_id: String,
    pub object_id: String,
    pub title: String,
    pub snippet: String,
}

#[tauri::command]
pub async fn fts_search(
    query: String,
    object_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<SearchResultDto>, String> {
    let q = SearchQuery {
        text: query,
        object_id,
    };

    let results = state.search_service.search(q).await?;

    Ok(results
        .into_iter()
        .map(|r| SearchResultDto {
            entry_id: r.entry_id,
            object_id: r.object_id,
            title: r.title,
            snippet: r.snippet,
        })
        .collect())
}
