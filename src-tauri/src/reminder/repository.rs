use super::models::{RecurrenceRule, Reminder, ReminderStatus};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};

#[async_trait]
pub trait ReminderRepository: Send + Sync {
    async fn save(&self, reminder: &Reminder) -> Result<(), String>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Reminder>, String>;
    async fn get_active_reminders(&self) -> Result<Vec<Reminder>, String>;
    async fn update_status(
        &self,
        id: &str,
        old_status: ReminderStatus,
        new_status: ReminderStatus,
    ) -> Result<bool, String>;
    async fn delete(&self, id: &str) -> Result<(), String>;
}

pub struct SqliteReminderRepository {
    pool: SqlitePool,
}

impl SqliteReminderRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReminderRepository for SqliteReminderRepository {
    async fn save(&self, reminder: &Reminder) -> Result<(), String> {
        let trigger_at_str = reminder.trigger_at.to_rfc3339();
        let created_at_str = reminder.created_at.to_rfc3339();
        let updated_at_str = reminder.updated_at.to_rfc3339();
        let completed_at_str = reminder.completed_at.map(|dt| dt.to_rfc3339());

        sqlx::query(
            r#"
            INSERT INTO reminders (id, entry_id, title, body, trigger_at, status, recurrence, created_at, updated_at, completed_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                entry_id = excluded.entry_id,
                title = excluded.title,
                body = excluded.body,
                trigger_at = excluded.trigger_at,
                status = excluded.status,
                recurrence = excluded.recurrence,
                updated_at = excluded.updated_at,
                completed_at = excluded.completed_at
            "#
        )
        .bind(&reminder.id)
        .bind(&reminder.entry_id)
        .bind(&reminder.title)
        .bind(&reminder.body)
        .bind(trigger_at_str)
        .bind(reminder.status.as_str())
        .bind(reminder.recurrence.as_str())
        .bind(created_at_str)
        .bind(updated_at_str)
        .bind(completed_at_str)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Reminder>, String> {
        let row = sqlx::query("SELECT * FROM reminders WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            let trigger_at_str: String = r.try_get("trigger_at").map_err(|e| e.to_string())?;
            let created_at_str: String = r.try_get("created_at").map_err(|e| e.to_string())?;
            let updated_at_str: String = r.try_get("updated_at").map_err(|e| e.to_string())?;
            let completed_at_str: Option<String> = r.try_get("completed_at").unwrap_or(None);

            Ok(Some(Reminder {
                id: r.try_get("id").map_err(|e| e.to_string())?,
                entry_id: r.try_get("entry_id").unwrap_or(None),
                title: r.try_get("title").map_err(|e| e.to_string())?,
                body: r.try_get("body").unwrap_or(None),
                trigger_at: DateTime::parse_from_rfc3339(&trigger_at_str)
                    .map_err(|e| e.to_string())?
                    .with_timezone(&Utc),
                recurrence: RecurrenceRule::parse(
                    &r.try_get::<String, _>("recurrence")
                        .map_err(|e| e.to_string())?,
                ),
                status: ReminderStatus::parse(
                    &r.try_get::<String, _>("status")
                        .map_err(|e| e.to_string())?,
                ),
                created_at: DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|e| e.to_string())?
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&updated_at_str)
                    .map_err(|e| e.to_string())?
                    .with_timezone(&Utc),
                completed_at: completed_at_str
                    .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                    .transpose()
                    .map_err(|e| e.to_string())?,
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_active_reminders(&self) -> Result<Vec<Reminder>, String> {
        let rows = sqlx::query("SELECT * FROM reminders WHERE status IN ('Pending', 'Scheduled')")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut results = Vec::new();
        for r in rows {
            let trigger_at_str: String = r.try_get("trigger_at").map_err(|e| e.to_string())?;
            let created_at_str: String = r.try_get("created_at").map_err(|e| e.to_string())?;
            let updated_at_str: String = r.try_get("updated_at").map_err(|e| e.to_string())?;
            let completed_at_str: Option<String> = r.try_get("completed_at").unwrap_or(None);

            results.push(Reminder {
                id: r.try_get("id").map_err(|e| e.to_string())?,
                entry_id: r.try_get("entry_id").unwrap_or(None),
                title: r.try_get("title").map_err(|e| e.to_string())?,
                body: r.try_get("body").unwrap_or(None),
                trigger_at: DateTime::parse_from_rfc3339(&trigger_at_str)
                    .map_err(|e| e.to_string())?
                    .with_timezone(&Utc),
                recurrence: RecurrenceRule::parse(
                    &r.try_get::<String, _>("recurrence")
                        .map_err(|e| e.to_string())?,
                ),
                status: ReminderStatus::parse(
                    &r.try_get::<String, _>("status")
                        .map_err(|e| e.to_string())?,
                ),
                created_at: DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|e| e.to_string())?
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&updated_at_str)
                    .map_err(|e| e.to_string())?
                    .with_timezone(&Utc),
                completed_at: completed_at_str
                    .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                    .transpose()
                    .map_err(|e| e.to_string())?,
            });
        }
        Ok(results)
    }

    async fn update_status(
        &self,
        id: &str,
        old_status: ReminderStatus,
        new_status: ReminderStatus,
    ) -> Result<bool, String> {
        let completed_clause = if new_status == ReminderStatus::Completed {
            ", completed_at = ?"
        } else {
            ""
        };
        let query_str = format!(
            "UPDATE reminders SET status = ?, updated_at = ? {} WHERE id = ? AND status = ?",
            completed_clause
        );

        let now = Utc::now().to_rfc3339();
        let completed_str = Utc::now().to_rfc3339();

        let q = sqlx::query(&query_str).bind(new_status.as_str()).bind(now);

        let q = if new_status == ReminderStatus::Completed {
            q.bind(completed_str)
        } else {
            q
        };

        let res = q
            .bind(id)
            .bind(old_status.as_str())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(res.rows_affected() > 0)
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM reminders WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
