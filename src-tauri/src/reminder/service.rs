use super::models::{RecurrenceRule, Reminder, ReminderStatus};
use super::repository::ReminderRepository;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub struct ReminderService {
    repository: Arc<dyn ReminderRepository>,
}

impl ReminderService {
    pub fn new(repository: Arc<dyn ReminderRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_reminder(
        &self,
        entry_id: Option<String>,
        title: String,
        body: Option<String>,
        trigger_at: chrono::DateTime<Utc>,
        recurrence: RecurrenceRule,
    ) -> Result<Reminder, String> {
        if title.trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }

        let reminder = Reminder {
            id: Uuid::new_v4().to_string(),
            entry_id,
            title,
            body,
            trigger_at,
            recurrence,
            status: ReminderStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            completed_at: None,
        };

        self.repository.save(&reminder).await?;
        Ok(reminder)
    }

    pub async fn cancel_reminder(&self, id: &str) -> Result<(), String> {
        // Atomic status transition for cancellation to avoid overwriting triggered states
        for old in &[
            ReminderStatus::Pending,
            ReminderStatus::Scheduled,
            ReminderStatus::Failed,
        ] {
            if self
                .repository
                .update_status(id, old.clone(), ReminderStatus::Cancelled)
                .await?
            {
                return Ok(());
            }
        }
        Err(
            "Reminder cannot be cancelled because it is already triggered, completed or cancelled"
                .to_string(),
        )
    }

    pub async fn complete_reminder(&self, id: &str) -> Result<bool, String> {
        if self.repository.find_by_id(id).await?.is_some() {
            for old in &[
                ReminderStatus::Triggered,
                ReminderStatus::Scheduled,
                ReminderStatus::Pending,
                ReminderStatus::Failed,
            ] {
                if self
                    .repository
                    .update_status(id, old.clone(), ReminderStatus::Completed)
                    .await?
                {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    pub async fn delete_reminder(&self, id: &str) -> Result<(), String> {
        self.repository.delete(id).await
    }

    pub async fn get_active_reminders(&self) -> Result<Vec<Reminder>, String> {
        self.repository.get_active_reminders().await
    }
}
