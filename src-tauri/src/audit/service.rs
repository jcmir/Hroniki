use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use crate::identity::error::IdentityError;
use super::models::AuditLogEntry;
use super::repository::AuditRepository;

pub struct AuditService {
    repository: Arc<dyn AuditRepository>,
}

impl AuditService {
    pub fn new(repository: Arc<dyn AuditRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_user_logs(&self, user_id: Option<&str>) -> Result<Vec<AuditLogEntry>, IdentityError> {
        self.repository.fetch_logs(user_id).await
    }

    pub async fn record_event(&self, user_id: Option<String>, event_type: &str, details: Option<String>) -> Result<(), IdentityError> {
        let entry = AuditLogEntry {
            id: Uuid::new_v4().to_string(),
            user_id,
            event_type: event_type.to_string(),
            details,
            created_at: Utc::now(),
        };
        self.repository.record_log(entry).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::SqliteIdentityRepository;
    use sqlx::SqlitePool;

    async fn create_test_pool() -> SqlitePool {
        let temp_dir = std::env::temp_dir();
        let db_file = temp_dir.join(format!("hroniki_test_audit_{}.sqlite", Uuid::new_v4()));
        let db_url = format!("sqlite://{}", db_file.to_string_lossy().replace('\\', "/"));
        let pool = crate::storage::connection::create_pool(&db_url).await.unwrap();
        crate::storage::migrations::run_migrations(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn audit_service_record_and_fetch() {
        let pool = create_test_pool().await;
        let repository = Arc::new(SqliteIdentityRepository::new(pool.clone()));
        let service = AuditService::new(repository);

        let user_id = Uuid::new_v4().to_string();
        
        // Seed user in database so foreign key check succeeds
        sqlx::query("INSERT INTO users (id, email, display_name, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&user_id)
            .bind("tester@audit.com")
            .bind("Tester")
            .bind("hash")
            .bind(chrono::Utc::now().to_rfc3339())
            .bind(chrono::Utc::now().to_rfc3339())
            .execute(&pool)
            .await
            .unwrap();

        // 1. Record an event for the user
        service.record_event(
            Some(user_id.clone()),
            "UserAuthenticated",
            Some(r#"{"device":"Desktop"}"#.to_string())
        ).await.unwrap();

        // 2. Record a system event (no user_id)
        service.record_event(
            None,
            "ArchiveExported",
            Some(r#"{"path":"/path/to/archive"}"#.to_string())
        ).await.unwrap();

        // 3. Fetch logs for user
        let user_logs = service.get_user_logs(Some(&user_id)).await.unwrap();
        assert_eq!(user_logs.len(), 1);
        assert_eq!(user_logs[0].event_type, "UserAuthenticated");
        assert_eq!(user_logs[0].details.as_deref(), Some(r#"{"device":"Desktop"}"#));

        // 4. Fetch all logs (system and users combined)
        let all_logs = service.get_user_logs(None).await.unwrap();
        assert_eq!(all_logs.len(), 2);

        pool.close().await;
    }
}
