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
    event_bus: Arc<crate::events::EventBus>,
}

impl LocalAccountProvider {
    pub fn new(repository: Arc<dyn IdentityRepository>, event_bus: Arc<crate::events::EventBus>) -> Self {
        Self { repository, event_bus }
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

        // Publish registration event
        self.event_bus.publish(crate::events::DomainEvent::UserRegistered {
            user_id: user.id.clone(),
            email: user.email.clone(),
        });

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
            // Publish authentication failure event
            self.event_bus.publish(crate::events::DomainEvent::UserAuthenticated {
                user_id: user.id.clone(),
                success: false,
            });
            return Err(IdentityError::InvalidPassword);
        }

        // Publish authentication success event
        self.event_bus.publish(crate::events::DomainEvent::UserAuthenticated {
            user_id: user.id.clone(),
            success: true,
        });

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

        // Publish session opened event
        self.event_bus.publish(crate::events::DomainEvent::SessionOpened {
            session_id: session.id.clone(),
            user_id: session.user_id.clone(),
            device_name: session.device_name.clone(),
        });

        Ok(session)
    }

    async fn invalidate_session(
        &self,
        session_id: &str,
    ) -> Result<(), IdentityError> {
        self.repository.delete_session(session_id).await?;

        // Publish session closed event
        self.event_bus.publish(crate::events::DomainEvent::SessionClosed {
            session_id: session_id.to_string(),
        });

        Ok(())
    }
}

