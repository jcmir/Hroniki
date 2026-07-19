use crate::platform::notifications::NotificationPlatform;
use async_trait::async_trait;

pub struct AndroidNotificationPlatform;

#[async_trait]
impl NotificationPlatform for AndroidNotificationPlatform {
    async fn show(&self, _title: &str, _body: Option<&str>) -> Result<(), String> {
        Err("Android notifications not implemented yet".to_string())
    }
}
