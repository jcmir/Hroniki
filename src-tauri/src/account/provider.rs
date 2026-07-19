use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use crate::identity::{
    models::{User, Session},
    repository::IdentityRepository,
    error::IdentityError,
};

#[async_trait]
pub trait AccountProvider: Send + Sync {
    async fn create_account(
        &self,
        email: Option<String>,
        display_name: Option<String>,
        password: &str,
    ) -> Result<User, IdentityError>;

    async fn authenticate(
        &self,
        email: &str,
        password: &str,
    ) -> Result<User, IdentityError>;

    async fn create_session(
        &self,
        user_id: &str,
        device_name: Option<String>,
    ) -> Result<Session, IdentityError>;

    async fn invalidate_session(
        &self,
        session_id: &str,
    ) -> Result<(), IdentityError>;
}

pub struct LocalAccountProvider {
    repository: Arc<dyn IdentityRepository>,
}

impl LocalAccountProvider {
    pub fn new(repository: Arc<dyn IdentityRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl AccountProvider for LocalAccountProvider {
    async fn create_account(
        &self,
        email: Option<String>,
        display_name: Option<String>,
        password: &str,
    ) -> Result<User, IdentityError> {
        if let Some(ref email_str) = email {
            if self.repository.find_by_email(email_str).await?.is_some() {
                return Err(IdentityError::UserExists);
            }
        }

        let user_id = Uuid::new_v4().to_string();
        let password_hash = crate::security::password::hash_password(password)
            .map_err(|e| IdentityError::Crypto(e))?;

        let user = User {
            id: user_id,
            email,
            display_name,
            created_at: Utc::now(),
        };

        self.repository.create_user(user.clone(), password_hash).await?;

        Ok(user)
    }

    async fn authenticate(
        &self,
        email: &str,
        password: &str,
    ) -> Result<User, IdentityError> {
        let (user, password_hash) = match self.repository.find_user_with_hash(email).await? {
            Some(res) => res,
            None => return Err(IdentityError::UserNotFound),
        };

        let is_valid = crate::security::password::verify_password(password, &password_hash)
            .map_err(|e| IdentityError::Crypto(e))?;

        if !is_valid {
            return Err(IdentityError::InvalidPassword);
        }

        Ok(user)
    }

    async fn create_session(
        &self,
        user_id: &str,
        device_name: Option<String>,
    ) -> Result<Session, IdentityError> {
        let session = Session {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            device_name,
            created_at: Utc::now(),
        };

        self.repository.create_session(session.clone()).await?;

        Ok(session)
    }

    async fn invalidate_session(
        &self,
        session_id: &str,
    ) -> Result<(), IdentityError> {
        self.repository.delete_session(session_id).await?;
        Ok(())
    }
}
