use async_trait::async_trait;

use super::models::User;
use super::error::IdentityError;

#[async_trait]
pub trait IdentityRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, IdentityError>;
    async fn create_user(&self, user: User, password_hash: String) -> Result<(), IdentityError>;
}
