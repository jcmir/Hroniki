use async_trait::async_trait;

#[async_trait]
pub trait NotificationPlatform: Send + Sync {
    async fn show(&self, title: &str, body: Option<&str>) -> Result<(), String>;
}
