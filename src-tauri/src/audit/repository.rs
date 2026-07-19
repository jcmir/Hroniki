use super::models::AuditLogEntry;
use crate::identity::error::IdentityError;
use async_trait::async_trait;

#[async_trait]
pub trait AuditRepository: Send + Sync {
    async fn record_log(&self, entry: AuditLogEntry) -> Result<(), IdentityError>;
    async fn fetch_logs(&self, user_id: Option<&str>) -> Result<Vec<AuditLogEntry>, IdentityError>;
}
