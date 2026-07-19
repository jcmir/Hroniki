use async_trait::async_trait;
use std::sync::Arc;

use super::models::Feature;
use super::repository::SubscriptionRepository;

#[async_trait]
pub trait FeatureProvider: Send + Sync {
    async fn is_enabled(&self, user_id: &str, feature: Feature) -> bool;
    async fn get_enabled_features(&self, user_id: &str) -> Vec<Feature>;
}

pub struct SubscriptionFeatureProvider {
    repository: Arc<dyn SubscriptionRepository>,
}

impl SubscriptionFeatureProvider {
    pub fn new(repository: Arc<dyn SubscriptionRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl FeatureProvider for SubscriptionFeatureProvider {
    async fn is_enabled(&self, user_id: &str, feature: Feature) -> bool {
        match self.repository.get_user_plan(user_id).await {
            Ok(plan) => plan.features().contains(&feature),
            Err(_) => false,
        }
    }

    async fn get_enabled_features(&self, user_id: &str) -> Vec<Feature> {
        match self.repository.get_user_plan(user_id).await {
            Ok(plan) => plan.features(),
            Err(_) => vec![],
        }
    }
}
