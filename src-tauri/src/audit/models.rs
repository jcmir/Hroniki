use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    pub id: String,
    pub user_id: Option<String>,
    pub event_type: String,
    pub details: Option<String>,
    pub created_at: DateTime<Utc>,
}
