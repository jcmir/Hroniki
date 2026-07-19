use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Row, SqlitePool};

use crate::identity::{
    error::IdentityError,
    models::{Session, User},
    repository::IdentityRepository,
};

pub struct SqliteIdentityRepository {
    pool: SqlitePool,
}

impl SqliteIdentityRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IdentityRepository for SqliteIdentityRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, IdentityError> {
        let row = sqlx::query(
            r#"
            SELECT
                id,
                email,
                display_name,
                created_at
            FROM users
            WHERE email = ?
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| IdentityError::Storage(e.to_string()))?;

        if let Some(r) = row {
            let id: String = r
                .try_get("id")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;
            let email: Option<String> = r
                .try_get("email")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;
            let display_name: Option<String> = r
                .try_get("display_name")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;
            let created_at_str: String = r
                .try_get("created_at")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;

            let created_at = created_at_str
                .parse()
                .map_err(|e| IdentityError::Storage(format!("Invalid date format: {}", e)))?;

            Ok(Some(User {
                id,
                email,
                display_name,
                created_at,
            }))
        } else {
            Ok(None)
        }
    }

    async fn find_user_with_hash(
        &self,
        email: &str,
    ) -> Result<Option<(User, String)>, IdentityError> {
        let row = sqlx::query(
            r#"
            SELECT
                id,
                email,
                display_name,
                password_hash,
                created_at
            FROM users
            WHERE email = ?
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| IdentityError::Storage(e.to_string()))?;

        if let Some(r) = row {
            let id: String = r
                .try_get("id")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;
            let email: Option<String> = r
                .try_get("email")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;
            let display_name: Option<String> = r
                .try_get("display_name")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;
            let password_hash: String = r
                .try_get("password_hash")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;
            let created_at_str: String = r
                .try_get("created_at")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;

            let created_at = created_at_str
                .parse()
                .map_err(|e| IdentityError::Storage(format!("Invalid date format: {}", e)))?;

            Ok(Some((
                User {
                    id,
                    email,
                    display_name,
                    created_at,
                },
                password_hash,
            )))
        } else {
            Ok(None)
        }
    }

    async fn create_user(&self, user: User, password_hash: String) -> Result<(), IdentityError> {
        sqlx::query(
            r#"
            INSERT INTO users
            (
                id,
                email,
                display_name,
                password_hash,
                created_at,
                updated_at
            )
            VALUES
            (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(user.id)
        .bind(user.email)
        .bind(user.display_name)
        .bind(password_hash)
        .bind(user.created_at.to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| IdentityError::Storage(e.to_string()))?;

        Ok(())
    }

    async fn create_session(&self, session: Session) -> Result<(), IdentityError> {
        sqlx::query(
            r#"
            INSERT INTO user_sessions
            (
                id,
                user_id,
                device_name,
                created_at,
                expires_at
            )
            VALUES
            (?, ?, ?, ?, ?)
            "#,
        )
        .bind(session.id)
        .bind(session.user_id)
        .bind(session.device_name)
        .bind(session.created_at.to_rfc3339())
        .bind((Utc::now() + chrono::Duration::days(30)).to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| IdentityError::Storage(e.to_string()))?;

        Ok(())
    }

    async fn find_session(&self, session_id: &str) -> Result<Option<Session>, IdentityError> {
        let row = sqlx::query(
            r#"
            SELECT
                id,
                user_id,
                device_name,
                created_at
            FROM user_sessions
            WHERE id = ?
            "#,
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| IdentityError::Storage(e.to_string()))?;

        if let Some(r) = row {
            let id: String = r
                .try_get("id")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;
            let user_id: String = r
                .try_get("user_id")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;
            let device_name: Option<String> = r
                .try_get("device_name")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;
            let created_at_str: String = r
                .try_get("created_at")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;

            let created_at = created_at_str
                .parse()
                .map_err(|e| IdentityError::Storage(format!("Invalid date format: {}", e)))?;

            Ok(Some(Session {
                id,
                user_id,
                device_name,
                created_at,
            }))
        } else {
            Ok(None)
        }
    }

    async fn delete_session(&self, session_id: &str) -> Result<(), IdentityError> {
        sqlx::query(
            r#"
            DELETE FROM user_sessions
            WHERE id = ?
            "#,
        )
        .bind(session_id)
        .execute(&self.pool)
        .await
        .map_err(|e| IdentityError::Storage(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl crate::features::repository::SubscriptionRepository for SqliteIdentityRepository {
    async fn get_user_plan(
        &self,
        user_id: &str,
    ) -> Result<crate::features::models::SubscriptionPlan, IdentityError> {
        let row = sqlx::query(
            r#"
            SELECT plan
            FROM subscriptions
            WHERE user_id = ? AND status = 'Active'
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| IdentityError::Storage(e.to_string()))?;

        if let Some(r) = row {
            let plan_str: String = r
                .try_get("plan")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;
            Ok(crate::features::models::SubscriptionPlan::parse(
                &plan_str,
            ))
        } else {
            Ok(crate::features::models::SubscriptionPlan::Free)
        }
    }

    async fn set_user_plan(
        &self,
        user_id: &str,
        plan: crate::features::models::SubscriptionPlan,
    ) -> Result<(), IdentityError> {
        sqlx::query(
            r#"
            INSERT INTO subscriptions
            (
                user_id,
                plan,
                status,
                updated_at
            )
            VALUES
            (?, ?, 'Active', ?)
            ON CONFLICT(user_id) DO UPDATE SET
                plan = excluded.plan,
                status = 'Active',
                updated_at = excluded.updated_at
            "#,
        )
        .bind(user_id)
        .bind(plan.as_str())
        .bind(Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| IdentityError::Storage(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl crate::audit::repository::AuditRepository for SqliteIdentityRepository {
    async fn record_log(
        &self,
        entry: crate::audit::models::AuditLogEntry,
    ) -> Result<(), IdentityError> {
        sqlx::query(
            r#"
            INSERT INTO audit_logs (id, user_id, event_type, details, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(entry.id)
        .bind(entry.user_id)
        .bind(entry.event_type)
        .bind(entry.details)
        .bind(entry.created_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| IdentityError::Storage(e.to_string()))?;

        Ok(())
    }

    async fn fetch_logs(
        &self,
        user_id: Option<&str>,
    ) -> Result<Vec<crate::audit::models::AuditLogEntry>, IdentityError> {
        let rows = if let Some(uid) = user_id {
            sqlx::query(
                r#"
                SELECT id, user_id, event_type, details, created_at
                FROM audit_logs
                WHERE user_id = ?
                ORDER BY created_at DESC
                "#,
            )
            .bind(uid)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| IdentityError::Storage(e.to_string()))?
        } else {
            sqlx::query(
                r#"
                SELECT id, user_id, event_type, details, created_at
                FROM audit_logs
                ORDER BY created_at DESC
                "#,
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| IdentityError::Storage(e.to_string()))?
        };

        let mut entries = Vec::new();
        for r in rows {
            let id: String = r
                .try_get("id")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;
            let user_id: Option<String> = r
                .try_get("user_id")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;
            let event_type: String = r
                .try_get("event_type")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;
            let details: Option<String> = r
                .try_get("details")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;
            let created_at_str: String = r
                .try_get("created_at")
                .map_err(|e| IdentityError::Storage(e.to_string()))?;
            let created_at = created_at_str
                .parse()
                .map_err(|e| IdentityError::Storage(format!("Invalid date format: {}", e)))?;

            entries.push(crate::audit::models::AuditLogEntry {
                id,
                user_id,
                event_type,
                details,
                created_at,
            });
        }

        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    async fn create_test_pool() -> SqlitePool {
        let temp_dir = std::env::temp_dir();
        let db_file = temp_dir.join(format!("hroniki_test_identity_{}.sqlite", Uuid::new_v4()));
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
    async fn creates_and_reads_user() {
        let pool = create_test_pool().await;
        let repo = SqliteIdentityRepository::new(pool.clone());

        let email = format!("test_{}@test.com", Uuid::new_v4());
        let user = User {
            id: Uuid::new_v4().to_string(),
            email: Some(email.clone()),
            display_name: Some("Test User".into()),
            created_at: Utc::now(),
        };

        repo.create_user(user.clone(), "hash".into()).await.unwrap();

        // Test basic find by email
        let loaded = repo.find_by_email(&email).await.unwrap().unwrap();

        assert_eq!(loaded.id, user.id);
        assert_eq!(loaded.email, user.email);

        // Test find user with hash
        let (user_loaded, hash) = repo.find_user_with_hash(&email).await.unwrap().unwrap();

        assert_eq!(user_loaded.id, user.id);
        assert_eq!(hash, "hash");

        // Clean up pool/file
        pool.close().await;
    }

    #[tokio::test]
    async fn session_lifecycle() {
        let pool = create_test_pool().await;
        let repo = SqliteIdentityRepository::new(pool.clone());

        let user = User {
            id: Uuid::new_v4().to_string(),
            email: Some(format!("user_{}@test.com", Uuid::new_v4())),
            display_name: Some("Session Tester".into()),
            created_at: Utc::now(),
        };
        repo.create_user(user.clone(), "pass".into()).await.unwrap();

        let session = Session {
            id: Uuid::new_v4().to_string(),
            user_id: user.id.clone(),
            device_name: Some("Rust Test Suite".into()),
            created_at: Utc::now(),
        };

        // Create
        repo.create_session(session.clone()).await.unwrap();

        // Read
        let loaded = repo.find_session(&session.id).await.unwrap().unwrap();
        assert_eq!(loaded.id, session.id);
        assert_eq!(loaded.user_id, session.user_id);
        assert_eq!(loaded.device_name, session.device_name);

        // Delete
        repo.delete_session(&session.id).await.unwrap();
        let deleted = repo.find_session(&session.id).await.unwrap();
        assert!(deleted.is_none());

        pool.close().await;
    }
}
