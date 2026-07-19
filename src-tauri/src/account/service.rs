use std::sync::Arc;

use super::provider::AccountProvider;
use crate::identity::{
    error::IdentityError,
    models::{Session, User},
};

pub struct AccountService {
    provider: Arc<dyn AccountProvider>,
}

impl AccountService {
    pub fn new(provider: Arc<dyn AccountProvider>) -> Self {
        Self { provider }
    }

    pub async fn create_account(
        &self,
        email: Option<String>,
        display_name: Option<String>,
        password: &str,
    ) -> Result<User, IdentityError> {
        self.provider
            .create_account(email, display_name, password)
            .await
    }

    pub async fn authenticate(&self, email: &str, password: &str) -> Result<User, IdentityError> {
        self.provider.authenticate(email, password).await
    }

    pub async fn open_session(
        &self,
        user_id: &str,
        device_name: Option<String>,
    ) -> Result<Session, IdentityError> {
        self.provider.create_session(user_id, device_name).await
    }

    pub async fn close_session(&self, session_id: &str) -> Result<(), IdentityError> {
        self.provider.invalidate_session(session_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::provider::LocalAccountProvider;
    use crate::storage::SqliteIdentityRepository;
    use sqlx::SqlitePool;
    use uuid::Uuid;

    async fn create_test_pool() -> SqlitePool {
        let temp_dir = std::env::temp_dir();
        let db_file = temp_dir.join(format!("hroniki_test_account_{}.sqlite", Uuid::new_v4()));
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
    async fn local_account_flow() {
        let pool = create_test_pool().await;
        let repository = Arc::new(SqliteIdentityRepository::new(pool.clone()));
        let event_bus = Arc::new(crate::events::EventBus::new());
        let provider = Arc::new(LocalAccountProvider::new(repository, event_bus));
        let service = AccountService::new(provider);

        let email = format!("user_{}@test.com", Uuid::new_v4());
        let password = "SuperSecretPassword123";

        // 1. Create account
        let user = service
            .create_account(Some(email.clone()), Some("Jane Doe".into()), password)
            .await
            .unwrap();

        assert_eq!(user.email, Some(email.clone()));
        assert_eq!(user.display_name, Some("Jane Doe".into()));

        // 2. Reject duplicate email registration
        let duplicate = service
            .create_account(Some(email.clone()), Some("Other".into()), password)
            .await;
        assert!(matches!(duplicate, Err(IdentityError::UserExists)));

        // 3. Authenticate with correct credentials
        let authenticated_user = service.authenticate(&email, password).await.unwrap();
        assert_eq!(authenticated_user.id, user.id);

        // 4. Reject wrong password
        let bad_auth = service.authenticate(&email, "wrong_pass").await;
        assert!(matches!(bad_auth, Err(IdentityError::InvalidPassword)));

        // 5. Open and close session
        let session = service
            .open_session(&user.id, Some("MacBook Pro".into()))
            .await
            .unwrap();
        assert_eq!(session.user_id, user.id);
        assert_eq!(session.device_name, Some("MacBook Pro".into()));

        service.close_session(&session.id).await.unwrap();

        pool.close().await;
    }
}
