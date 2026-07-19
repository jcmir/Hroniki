use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Row, SqlitePool};

use crate::identity::{
    models::User,
    repository::IdentityRepository,
    error::IdentityError,
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
    async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<Option<User>, IdentityError> {
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
            let id: String = r.try_get("id").map_err(|e| IdentityError::Storage(e.to_string()))?;
            let email: Option<String> = r.try_get("email").map_err(|e| IdentityError::Storage(e.to_string()))?;
            let display_name: Option<String> = r.try_get("display_name").map_err(|e| IdentityError::Storage(e.to_string()))?;
            let created_at_str: String = r.try_get("created_at").map_err(|e| IdentityError::Storage(e.to_string()))?;
            
            let created_at = created_at_str.parse().unwrap_or_else(|_| Utc::now());

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

    async fn create_user(
        &self,
        user: User,
        password_hash: String,
    ) -> Result<(), IdentityError> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    async fn create_test_pool() -> SqlitePool {
        let temp_dir = std::env::temp_dir();
        let db_file = temp_dir.join(format!("hroniki_test_identity_{}.sqlite", Uuid::new_v4()));
        let db_url = format!("sqlite://{}", db_file.to_string_lossy().replace('\\', "/"));
        let pool = crate::storage::connection::create_pool(&db_url).await.unwrap();
        crate::storage::migrations::run_migrations(&pool).await.unwrap();
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

        repo.create_user(user.clone(), "hash".into())
            .await
            .unwrap();

        let loaded = repo.find_by_email(&email)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(loaded.id, user.id);
        assert_eq!(loaded.email, user.email);
        assert_eq!(loaded.display_name, user.display_name);
        
        // Clean up pool/file
        pool.close().await;
    }
}
