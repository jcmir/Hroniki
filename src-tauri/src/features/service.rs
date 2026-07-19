use std::sync::Arc;

use super::models::{Feature, SubscriptionPlan};
use super::provider::FeatureProvider;
use super::repository::SubscriptionRepository;
use crate::identity::error::IdentityError;

pub struct FeatureService {
    provider: Arc<dyn FeatureProvider>,
    repository: Arc<dyn SubscriptionRepository>,
}

impl FeatureService {
    pub fn new(
        provider: Arc<dyn FeatureProvider>,
        repository: Arc<dyn SubscriptionRepository>,
    ) -> Self {
        Self {
            provider,
            repository,
        }
    }

    pub async fn is_enabled(&self, user_id: &str, feature: Feature) -> bool {
        self.provider.is_enabled(user_id, feature).await
    }

    pub async fn get_enabled_features(&self, user_id: &str) -> Vec<Feature> {
        self.provider.get_enabled_features(user_id).await
    }

    pub async fn get_user_plan(&self, user_id: &str) -> Result<SubscriptionPlan, IdentityError> {
        self.repository.get_user_plan(user_id).await
    }

    pub async fn update_user_plan(
        &self,
        user_id: &str,
        plan: SubscriptionPlan,
    ) -> Result<(), IdentityError> {
        self.repository.set_user_plan(user_id, plan).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::provider::SubscriptionFeatureProvider;
    use crate::storage::SqliteIdentityRepository;
    use sqlx::SqlitePool;
    use uuid::Uuid;

    async fn create_test_pool() -> SqlitePool {
        let temp_dir = std::env::temp_dir();
        let db_file = temp_dir.join(format!("hroniki_test_features_{}.sqlite", Uuid::new_v4()));
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
    async fn feature_gate_flows() {
        let pool = create_test_pool().await;
        let repository = Arc::new(SqliteIdentityRepository::new(pool.clone()));

        // Seed a test user
        let user_id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO users (id, email, display_name, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&user_id)
            .bind(format!("user_{}@test.com", Uuid::new_v4()))
            .bind("Tester")
            .bind("hash")
            .bind(chrono::Utc::now().to_rfc3339())
            .bind(chrono::Utc::now().to_rfc3339())
            .execute(&pool)
            .await
            .unwrap();

        let provider = Arc::new(SubscriptionFeatureProvider::new(repository.clone()));
        let service = FeatureService::new(provider, repository);

        // 1. By default, user should be on Free plan
        let initial_plan = service.get_user_plan(&user_id).await.unwrap();
        assert_eq!(initial_plan, SubscriptionPlan::Free);

        // 2. Test free plan features
        assert!(service.is_enabled(&user_id, Feature::ExportBackup).await);
        assert!(!service.is_enabled(&user_id, Feature::AI).await);
        assert!(!service.is_enabled(&user_id, Feature::CloudSync).await);
        assert!(!service.is_enabled(&user_id, Feature::FamilyArchive).await);

        // 3. Upgrade to Pro
        service
            .update_user_plan(&user_id, SubscriptionPlan::Pro)
            .await
            .unwrap();
        let pro_plan = service.get_user_plan(&user_id).await.unwrap();
        assert_eq!(pro_plan, SubscriptionPlan::Pro);

        // 4. Test Pro features
        assert!(service.is_enabled(&user_id, Feature::ExportBackup).await);
        assert!(
            service
                .is_enabled(&user_id, Feature::UnlimitedObjects)
                .await
        );
        assert!(service.is_enabled(&user_id, Feature::AI).await);
        assert!(service.is_enabled(&user_id, Feature::CloudSync).await);
        assert!(!service.is_enabled(&user_id, Feature::FamilyArchive).await);

        // 5. Upgrade to Family
        service
            .update_user_plan(&user_id, SubscriptionPlan::Family)
            .await
            .unwrap();
        let family_plan = service.get_user_plan(&user_id).await.unwrap();
        assert_eq!(family_plan, SubscriptionPlan::Family);
        assert!(service.is_enabled(&user_id, Feature::FamilyArchive).await);

        pool.close().await;
    }
}
