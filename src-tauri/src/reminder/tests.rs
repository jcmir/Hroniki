use async_trait::async_trait;
use chrono::{Duration, Utc};
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use super::models::{RecurrenceRule, ReminderStatus};
use super::provider::NotificationProvider;
use super::repository::{ReminderRepository, SqliteReminderRepository};
use super::scheduler::ReminderScheduler;
use super::service::ReminderService;

struct MockNotificationProvider {
    notifications: Arc<Mutex<Vec<(String, Option<String>)>>>,
    should_fail: bool,
}

impl MockNotificationProvider {
    fn new(should_fail: bool) -> Self {
        Self {
            notifications: Arc::new(Mutex::new(Vec::new())),
            should_fail,
        }
    }
}

#[async_trait]
impl NotificationProvider for MockNotificationProvider {
    async fn send(&self, title: &str, body: Option<&str>) -> Result<(), String> {
        if self.should_fail {
            return Err("Network Timeout".to_string());
        }
        let mut list = self.notifications.lock().await;
        list.push((title.to_string(), body.map(|s| s.to_string())));
        Ok(())
    }
}

async fn create_test_pool() -> SqlitePool {
    let temp_dir = std::env::temp_dir();
    let db_file = temp_dir.join(format!("hroniki_test_reminders_{}.sqlite", Uuid::new_v4()));
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
async fn test_create_and_complete_reminder() {
    let pool = create_test_pool().await;
    let repo = Arc::new(SqliteReminderRepository::new(pool.clone()));
    let service = ReminderService::new(repo.clone());

    let trigger = Utc::now() + Duration::hours(1);
    let reminder = service
        .create_reminder(
            None,
            "Test Title".to_string(),
            Some("Test Body".to_string()),
            trigger,
            RecurrenceRule::Once,
        )
        .await
        .unwrap();

    assert_eq!(reminder.title, "Test Title");
    assert_eq!(reminder.status, ReminderStatus::Pending);

    // Complete it
    let res = service.complete_reminder(&reminder.id).await.unwrap();
    assert!(res);

    let updated = repo.find_by_id(&reminder.id).await.unwrap().unwrap();
    assert_eq!(updated.status, ReminderStatus::Completed);
    assert!(updated.completed_at.is_some());

    pool.close().await;
}

#[tokio::test]
async fn test_scheduler_triggers_reminder() {
    let pool = create_test_pool().await;
    let repo = Arc::new(SqliteReminderRepository::new(pool.clone()));
    let mock_provider = Arc::new(MockNotificationProvider::new(false));
    let service = ReminderService::new(repo.clone());

    let trigger = Utc::now() - Duration::minutes(1);
    let reminder = service
        .create_reminder(
            None,
            "Due Alert".to_string(),
            Some("Body text".to_string()),
            trigger,
            RecurrenceRule::Once,
        )
        .await
        .unwrap();

    let scheduler = ReminderScheduler::new(repo.clone(), mock_provider.clone());
    scheduler.start();

    tokio::time::sleep(tokio::time::Duration::from_secs(11)).await;

    // Check status in DB (should be triggered)
    let updated = repo.find_by_id(&reminder.id).await.unwrap().unwrap();
    assert_eq!(updated.status, ReminderStatus::Triggered);

    let list = mock_provider.notifications.lock().await;
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].0, "Due Alert");

    pool.close().await;
}

#[tokio::test]
async fn test_provider_failed_remains_failed_for_retry() {
    let pool = create_test_pool().await;
    let repo = Arc::new(SqliteReminderRepository::new(pool.clone()));
    let mock_provider = Arc::new(MockNotificationProvider::new(true)); // Configured to fail
    let service = ReminderService::new(repo.clone());

    let trigger = Utc::now() - Duration::minutes(1);
    let reminder = service
        .create_reminder(
            None,
            "Failed Alert".to_string(),
            None,
            trigger,
            RecurrenceRule::Once,
        )
        .await
        .unwrap();

    let scheduler = ReminderScheduler::new(repo.clone(), mock_provider.clone());
    scheduler.start();

    tokio::time::sleep(tokio::time::Duration::from_secs(11)).await;

    // Should rollback to Failed state (ready to poll and retry again later)
    let updated = repo.find_by_id(&reminder.id).await.unwrap().unwrap();
    assert_eq!(updated.status, ReminderStatus::Failed);

    pool.close().await;
}

#[tokio::test]
async fn test_recurrence_rule_parsing() {
    let once = RecurrenceRule::parse("Once").unwrap();
    assert_eq!(once, RecurrenceRule::Once);

    let every_14 = RecurrenceRule::parse("EveryNDays:14").unwrap();
    assert_eq!(every_14, RecurrenceRule::EveryNDays(14));

    let err = RecurrenceRule::parse("InvalidRule");
    assert!(err.is_err());
}

#[tokio::test]
async fn test_cancel_already_completed_fails() {
    let pool = create_test_pool().await;
    let repo = Arc::new(SqliteReminderRepository::new(pool.clone()));
    let service = ReminderService::new(repo.clone());

    let trigger = Utc::now() + Duration::hours(1);
    let reminder = service
        .create_reminder(
            None,
            "To Cancel".to_string(),
            None,
            trigger,
            RecurrenceRule::Once,
        )
        .await
        .unwrap();

    // Complete it
    service.complete_reminder(&reminder.id).await.unwrap();

    // Try to cancel (should fail)
    let cancel_res = service.cancel_reminder(&reminder.id).await;
    assert!(cancel_res.is_err());

    pool.close().await;
}

#[tokio::test]
async fn test_create_reminder_validation() {
    let pool = create_test_pool().await;
    let repo = Arc::new(SqliteReminderRepository::new(pool.clone()));
    let service = ReminderService::new(repo.clone());

    let trigger = Utc::now() + Duration::hours(1);
    let res = service
        .create_reminder(None, "   ".to_string(), None, trigger, RecurrenceRule::Once)
        .await;

    assert!(res.is_err()); // Blank title not allowed

    pool.close().await;
}
