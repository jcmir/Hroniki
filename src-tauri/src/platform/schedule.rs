use async_trait::async_trait;

#[async_trait]
pub trait SchedulePlatform: Send + Sync {
    async fn schedule_exact(&self, alarm_id: &str, trigger_at_ms: u64) -> Result<(), String>;
    async fn cancel_alarm(&self, alarm_id: &str) -> Result<(), String>;
}
