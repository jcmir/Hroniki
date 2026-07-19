use crate::platform::notifications::NotificationPlatform;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationChannel {
    pub id: String,
    pub name: String,
    pub importance: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostedNotification {
    pub title: String,
    pub body: Option<String>,
    pub channel_id: String,
}

pub struct AndroidNotificationPlatform {
    default_channel_id: String,
    channels: Arc<Mutex<HashMap<String, NotificationChannel>>>,
    posted_notifications: Arc<Mutex<Vec<PostedNotification>>>,
}

impl Default for AndroidNotificationPlatform {
    fn default() -> Self {
        Self::new("default_channel", "General Notifications", 3)
    }
}

impl AndroidNotificationPlatform {
    pub fn new(default_channel_id: &str, default_channel_name: &str, importance: u8) -> Self {
        let default_channel = NotificationChannel {
            id: default_channel_id.to_string(),
            name: default_channel_name.to_string(),
            importance,
        };
        let mut channels = HashMap::new();
        channels.insert(default_channel_id.to_string(), default_channel);

        Self {
            default_channel_id: default_channel_id.to_string(),
            channels: Arc::new(Mutex::new(channels)),
            posted_notifications: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn create_channel(&self, id: &str, name: &str, importance: u8) {
        let channel = NotificationChannel {
            id: id.to_string(),
            name: name.to_string(),
            importance,
        };
        let mut map = self.channels.lock().await;
        map.insert(id.to_string(), channel);
    }

    pub async fn get_channel(&self, id: &str) -> Option<NotificationChannel> {
        let map = self.channels.lock().await;
        map.get(id).cloned()
    }

    pub async fn posted_count(&self) -> usize {
        let list = self.posted_notifications.lock().await;
        list.len()
    }

    pub async fn show_on_channel(
        &self,
        title: &str,
        body: Option<&str>,
        channel_id: &str,
    ) -> Result<(), String> {
        let map = self.channels.lock().await;
        if !map.contains_key(channel_id) {
            return Err(format!(
                "NotificationChannel '{}' does not exist",
                channel_id
            ));
        }
        drop(map);

        let mut list = self.posted_notifications.lock().await;
        list.push(PostedNotification {
            title: title.to_string(),
            body: body.map(|s| s.to_string()),
            channel_id: channel_id.to_string(),
        });
        tracing::info!(
            "[AndroidNotificationPlatform] Delivered notification on channel '{}': title='{}', body='{:?}'",
            channel_id, title, body
        );
        Ok(())
    }
}

#[async_trait]
impl NotificationPlatform for AndroidNotificationPlatform {
    async fn show(&self, title: &str, body: Option<&str>) -> Result<(), String> {
        self.show_on_channel(title, body, &self.default_channel_id)
            .await
    }
}
