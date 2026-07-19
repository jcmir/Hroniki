use async_trait::async_trait;

use super::models::{User, Session};
use super::error::IdentityError;

#[async_trait]
pub trait IdentityRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, IdentityError>;
    async fn find_user_with_hash(&self, email: &str) -> Result<Option<(User, String)>, IdentityError>;
    async fn create_user(&self, user: User, password_hash: String) -> Result<(), IdentityError>;

    // Session operations
    async fn create_session(&self, session: Session) -> Result<(), IdentityError>;
    async fn find_session(&self, session_id: &str) -> Result<Option<Session>, IdentityError>;
    async fn delete_session(&self, session_id: &str) -> Result<(), IdentityError>;
}
