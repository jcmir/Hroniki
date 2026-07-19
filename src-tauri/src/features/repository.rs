use async_trait::async_trait;

use super::models::SubscriptionPlan;
use crate::identity::error::IdentityError;

#[async_trait]
pub trait SubscriptionRepository: Send + Sync {
    async fn get_user_plan(&self, user_id: &str) -> Result<SubscriptionPlan, IdentityError>;
    async fn set_user_plan(
        &self,
        user_id: &str,
        plan: SubscriptionPlan,
    ) -> Result<(), IdentityError>;
}
