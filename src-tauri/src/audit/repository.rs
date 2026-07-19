use async_trait::async_trait;
use crate::identity::error::IdentityError;
use super::models::AuditLogEntry;

#[async_trait]
pub trait AuditRepository: Send + Sync {
    async fn record_log(&self, entry: AuditLogEntry) -> Result<(), IdentityError>;
    async fn fetch_logs(&self, user_id: Option<&str>) -> Result<Vec<AuditLogEntry>, IdentityError>;
}
