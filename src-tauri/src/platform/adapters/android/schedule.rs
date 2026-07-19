use crate::platform::schedule::SchedulePlatform;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AndroidSchedulePlatform {
    scheduled_alarms: Arc<Mutex<HashMap<String, u64>>>,
}

impl Default for AndroidSchedulePlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl AndroidSchedulePlatform {
    pub fn new() -> Self {
        Self {
            scheduled_alarms: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn is_scheduled(&self, alarm_id: &str) -> bool {
        let map = self.scheduled_alarms.lock().await;
        map.contains_key(alarm_id)
    }
}

#[async_trait]
impl SchedulePlatform for AndroidSchedulePlatform {
    async fn schedule_exact(&self, alarm_id: &str, trigger_at_ms: u64) -> Result<(), String> {
        let mut map = self.scheduled_alarms.lock().await;
        map.insert(alarm_id.to_string(), trigger_at_ms);
        tracing::info!(
            "[AndroidSchedulePlatform] Scheduled exact alarm '{}' for ms: {}",
            alarm_id,
            trigger_at_ms
        );
        Ok(())
    }

    async fn cancel_alarm(&self, alarm_id: &str) -> Result<(), String> {
        let mut map = self.scheduled_alarms.lock().await;
        if map.remove(alarm_id).is_some() {
            tracing::info!("[AndroidSchedulePlatform] Cancelled alarm '{}'", alarm_id);
            Ok(())
        } else {
            Err(format!("Alarm '{}' not found to cancel", alarm_id))
        }
    }
}
