use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{Utc, Duration};
use uuid::Uuid;
use sqlx::{SqlitePool, Row};
use async_trait::async_trait;

use super::models::{Reminder, ReminderStatus, RecurrenceRule};
use super::repository::{ReminderRepository, SqliteReminderRepository};
use super::provider::NotificationProvider;
use super::scheduler::ReminderScheduler;

// 1. Mock provider that fails a specified number of times then succeeds
struct TransientFailProvider {
    failure_count: Arc<Mutex<usize>>,
    success_count: Arc<Mutex<usize>>,
    target_failures: usize,
}

impl TransientFailProvider {
    fn new(target_failures: usize) -> Self {
        Self {
            failure_count: Arc::new(Mutex::new(0)),
            success_count: Arc::new(Mutex::new(0)),
            target_failures,
        }
    }
}

#[async_trait]
impl NotificationProvider for TransientFailProvider {
    async fn send(&self, _title: &str, _body: Option<&str>) -> Result<(), String> {
        let mut fail_lock = self.failure_count.lock().await;
        if *fail_lock < self.target_failures {
            *fail_lock += 1;
            return Err("Transient Network Error".to_string());
        }
        let mut success_lock = self.success_count.lock().await;
        *success_lock += 1;
        Ok(())
    }
}

async fn create_test_pool() -> SqlitePool {
    let temp_dir = std::env::temp_dir();
    let db_file = temp_dir.join(format!("hroniki_test_hardening_{}.sqlite", Uuid::new_v4()));
    let db_url = format!("sqlite://{}", db_file.to_string_lossy().replace('\\', "/"));
    let pool = crate::storage::connection::create_pool(&db_url).await.unwrap();
    crate::storage::migrations::run_migrations(&pool).await.unwrap();
    pool
}

// --- Hardening Tests ---

#[tokio::test]
async fn test_concurrent_scheduler_claims() {
    let pool = create_test_pool().await;
    let repo = Arc::new(SqliteReminderRepository::new(pool.clone()));

    // Create a pending reminder
    let reminder = Reminder {
        id: Uuid::new_v4().to_string(),
        entry_id: None,
        title: "Concurrent Title".to_string(),
        body: None,
        trigger_at: Utc::now() - Duration::minutes(1),
        recurrence: RecurrenceRule::Once,
        status: ReminderStatus::Pending,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        completed_at: None,
    };
    repo.save(&reminder).await.unwrap();

    // Spawn 10 concurrent threads trying to claim the same reminder
    let mut join_handles = vec![];
    for _ in 0..10 {
        let r_clone = repo.clone();
        let id_clone = reminder.id.clone();
        join_handles.push(tokio::spawn(async move {
            r_clone.update_status(&id_clone, ReminderStatus::Pending, ReminderStatus::Triggered).await
        }));
    }

    let mut success_claims = 0;
    for handle in join_handles {
        if let Ok(Ok(true)) = handle.await {
            success_claims += 1;
        }
    }

    // Exactly one thread must successfully transition the status
    assert_eq!(success_claims, 1);

    pool.close().await;
}

#[tokio::test]
async fn test_scheduler_recovery_on_transient_failure() {
    let pool = create_test_pool().await;
    let repo = Arc::new(SqliteReminderRepository::new(pool.clone()));
    let mock_provider = Arc::new(TransientFailProvider::new(2)); // Fails twice

    let reminder = Reminder {
        id: Uuid::new_v4().to_string(),
        entry_id: None,
        title: "Transient Title".to_string(),
        body: None,
        trigger_at: Utc::now() - Duration::minutes(1),
        recurrence: RecurrenceRule::Once,
        status: ReminderStatus::Pending,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        completed_at: None,
    };
    repo.save(&reminder).await.unwrap();

    let scheduler = ReminderScheduler::new(repo.clone(), mock_provider.clone());
    scheduler.start();

    // Run scheduler and wait enough time for 3 ticks to occur (at 0s, 10s, and 20s)
    tokio::time::sleep(tokio::time::Duration::from_secs(25)).await;

    // Check final status in DB (should be Triggered after recovery)
    let status = repo.find_by_id(&reminder.id).await.unwrap().unwrap().status;
    assert_eq!(status, ReminderStatus::Triggered);

    // Verify correct attempt counts
    assert_eq!(*mock_provider.failure_count.lock().await, 2);
    assert_eq!(*mock_provider.success_count.lock().await, 1);

    pool.close().await;
}

#[tokio::test]
async fn test_strict_parsing_failures() {
    // Test invalid status values
    assert!(ReminderStatus::parse("Triggred").is_err());
    assert!(ReminderStatus::parse("").is_err());

    // Test invalid recurrence rules
    assert!(RecurrenceRule::parse("EveryNDays:").is_err());
    assert!(RecurrenceRule::parse("EveryNDays:abc").is_err());
    assert!(RecurrenceRule::parse("OnceMore").is_err());
}

#[tokio::test]
async fn test_migration_0009_preserves_custom_repeat_days() {
    // Create an ephemeral DB file to test sequential migrations manually
    let temp_dir = std::env::temp_dir();
    let db_file = temp_dir.join(format!("hroniki_migration_test_{}.sqlite", Uuid::new_v4()));
    let db_url = format!("sqlite://{}", db_file.to_string_lossy().replace('\\', "/"));
    
    // Create database file manually
    std::fs::File::create(&db_file).unwrap();
    let pool = crate::storage::connection::create_pool(&db_url).await.unwrap();

    // Run migrations 1 to 7 manually to set up old schema
    // 0001_initial.sql
    sqlx::query("CREATE TABLE categories (id TEXT PRIMARY KEY NOT NULL, name TEXT NOT NULL, created_at TEXT NOT NULL)")
        .execute(&pool).await.unwrap();
    // 0002_photos.sql
    sqlx::query("CREATE TABLE objects (id TEXT PRIMARY KEY NOT NULL, category_id TEXT NOT NULL, name TEXT NOT NULL, description TEXT, created_at TEXT NOT NULL, FOREIGN KEY(category_id) REFERENCES categories(id) ON DELETE CASCADE)")
        .execute(&pool).await.unwrap();
    sqlx::query("CREATE TABLE entries (id TEXT PRIMARY KEY NOT NULL, object_id TEXT NOT NULL, occurred_at TEXT NOT NULL, title TEXT NOT NULL, description TEXT, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, FOREIGN KEY(object_id) REFERENCES objects(id) ON DELETE CASCADE)")
        .execute(&pool).await.unwrap();
    // 0003_add_reminders.sql (Old reminders schema with repeat_days)
    sqlx::query("CREATE TABLE reminders (id TEXT PRIMARY KEY NOT NULL, entry_id TEXT NOT NULL, trigger_at TEXT NOT NULL, status TEXT NOT NULL, repeat_days INTEGER, completed_at TEXT, FOREIGN KEY(entry_id) REFERENCES entries(id) ON DELETE CASCADE)")
        .execute(&pool).await.unwrap();

    // Seed test category, object, entry
    sqlx::query("INSERT INTO categories (id, name, created_at) VALUES ('cat-1', 'Home', '2026-07-19T12:00:00Z')")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO objects (id, category_id, name, description, created_at) VALUES ('obj-1', 'cat-1', 'Plumbing', 'Plumbing checks', '2026-07-19T12:00:00Z')")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO entries (id, object_id, occurred_at, title, description, created_at, updated_at) VALUES ('ent-1', 'obj-1', '2026-07-19T12:00:00Z', 'Leak fixed', 'Leak fixed', '2026-07-19T12:00:00Z', '2026-07-19T12:00:00Z')")
        .execute(&pool).await.unwrap();

    // Seed old reminder with repeat_days = 45 (custom recurrence)
    sqlx::query("INSERT INTO reminders (id, entry_id, trigger_at, status, repeat_days) VALUES ('rem-1', 'ent-1', '2026-07-20T12:00:00Z', 'Scheduled', 45)")
        .execute(&pool).await.unwrap();

    // Now run migration 0009_reminders_v2 manually
    let migration_sql = include_str!("../../migrations/0009_reminders_v2.sql");
    
    // We execute the migration queries. SQLite requires executing batch scripts as multiple statements or via transaction block
    // sqlx execute will run multiple semicolon-separated statements sequentially.
    sqlx::query(migration_sql).execute(&pool).await.unwrap();

    // Load migrated reminder and check that recurrence contains "EveryNDays:45"
    let row = sqlx::query("SELECT recurrence FROM reminders WHERE id = 'rem-1'")
        .fetch_one(&pool)
        .await
        .unwrap();
    let recurrence_str: String = row.try_get("recurrence").unwrap();
    assert_eq!(recurrence_str, "EveryNDays:45");

    pool.close().await;
    let _ = std::fs::remove_file(&db_file);
}
