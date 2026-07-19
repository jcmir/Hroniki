#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub text: String,
    pub object_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub entry_id: String,
    pub object_id: String,
    pub title: String,
    pub snippet: String,
}
