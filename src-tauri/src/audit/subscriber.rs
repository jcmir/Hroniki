use super::service::AuditService;
use crate::events::{DomainEvent, EventBus};
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct AuditSubscriber {
    event_bus: Arc<EventBus>,
    audit_service: Arc<AuditService>,
}

impl AuditSubscriber {
    pub fn new(event_bus: Arc<EventBus>, audit_service: Arc<AuditService>) -> Self {
        Self {
            event_bus,
            audit_service,
        }
    }

    pub fn start(self) {
        let mut rx = self.event_bus.subscribe();
        let service = self.audit_service.clone();

        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(event) => {
                        let result = match event {
                            DomainEvent::UserRegistered { user_id, email } => {
                                let details = format!(
                                    r#"{{"email":{}}}"#,
                                    email
                                        .map(|e| format!("\"{}\"", e))
                                        .unwrap_or_else(|| "null".to_string())
                                );
                                service
                                    .record_event(Some(user_id), "UserRegistered", Some(details))
                                    .await
                            }
                            DomainEvent::UserAuthenticated { user_id, success } => {
                                let details = format!(r#"{{"success":{}}}"#, success);
                                service
                                    .record_event(Some(user_id), "UserAuthenticated", Some(details))
                                    .await
                            }
                            DomainEvent::SessionOpened {
                                session_id,
                                user_id,
                                device_name,
                            } => {
                                let details = format!(
                                    r#"{{"session_id":"{}","device_name":{}}}"#,
                                    session_id,
                                    device_name
                                        .map(|d| format!("\"{}\"", d))
                                        .unwrap_or_else(|| "null".to_string())
                                );
                                service
                                    .record_event(Some(user_id), "SessionOpened", Some(details))
                                    .await
                            }
                            DomainEvent::SessionClosed { session_id } => {
                                let details = format!(r#"{{"session_id":"{}"}}"#, session_id);
                                service
                                    .record_event(None, "SessionClosed", Some(details))
                                    .await
                            }
                            DomainEvent::ArchiveExported { user_id, path } => {
                                let details =
                                    format!(r#"{{"path":"{}"}}"#, path.replace('\\', "/"));
                                service
                                    .record_event(user_id, "ArchiveExported", Some(details))
                                    .await
                            }
                            DomainEvent::ArchiveImported { user_id, success } => {
                                let details = format!(r#"{{"success":{}}}"#, success);
                                service
                                    .record_event(user_id, "ArchiveImported", Some(details))
                                    .await
                            }
                            DomainEvent::PlanUpdated {
                                user_id,
                                plan,
                                updated_at,
                            } => {
                                let details = format!(
                                    r#"{{"plan":"{}","updated_at":"{}"}}"#,
                                    plan, updated_at
                                );
                                service
                                    .record_event(Some(user_id), "PlanUpdated", Some(details))
                                    .await
                            }
                            _ => Ok(()),
                        };

                        if let Err(e) = result {
                            eprintln!("Failed to write audit log: {:?}", e);
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(skipped)) => {
                        eprintln!("AuditSubscriber lagged, skipped {} events", skipped);
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::SqliteIdentityRepository;
    use sqlx::SqlitePool;
    use uuid::Uuid;

    async fn create_test_pool() -> SqlitePool {
        let temp_dir = std::env::temp_dir();
        let db_file = temp_dir.join(format!("hroniki_test_audit_sub_{}.sqlite", Uuid::new_v4()));
        let db_url = format!("sqlite://{}", db_file.to_string_lossy().replace('\\', "/"));
        let pool = crate::storage::connection::create_pool(&db_url)
            .await
            .unwrap();
        crate::storage::migrations::run_migrations(&pool)
            .await
            .unwrap();
        pool
    }

    #[tokio::test]
    async fn audit_subscriber_integration_flow() {
        let pool = create_test_pool().await;
        let repository = Arc::new(SqliteIdentityRepository::new(pool.clone()));
        let audit_service = Arc::new(AuditService::new(repository));
        let event_bus = Arc::new(EventBus::new());

        let subscriber = AuditSubscriber::new(event_bus.clone(), audit_service.clone());
        subscriber.start();

        let user_id = Uuid::new_v4().to_string();

        // Seed user to satisfy foreign keys
        sqlx::query("INSERT INTO users (id, email, display_name, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&user_id)
            .bind("subscriber@test.com")
            .bind("SubscriberTester")
            .bind("hash")
            .bind(chrono::Utc::now().to_rfc3339())
            .bind(chrono::Utc::now().to_rfc3339())
            .execute(&pool)
            .await
            .unwrap();

        // 1. Publish Event
        event_bus.publish(DomainEvent::UserAuthenticated {
            user_id: user_id.clone(),
            success: true,
        });

        // 2. Wait slightly for async background task execution
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;

        // 3. Assert log is created in the database!
        let logs = audit_service.get_user_logs(Some(&user_id)).await.unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].event_type, "UserAuthenticated");
        assert_eq!(logs[0].details.as_deref(), Some(r#"{"success":true}"#));

        pool.close().await;
    }
}
