use async_trait::async_trait;

#[async_trait]
pub trait NotificationProvider: Send + Sync {
    async fn send(&self, title: &str, body: Option<&str>) -> Result<(), String>;
}

pub struct DummyNotificationProvider;

#[async_trait]
impl NotificationProvider for DummyNotificationProvider {
    async fn send(&self, title: &str, body: Option<&str>) -> Result<(), String> {
        println!("[NOTIFICATION] Title: '{}', Body: '{:?}'", title, body);
        Ok(())
    }
}
