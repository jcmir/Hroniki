use super::models::{RecurrenceRule, Reminder, ReminderStatus};
use super::provider::NotificationProvider;
use super::repository::ReminderRepository;
use chrono::Utc;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub struct ReminderScheduler {
    repository: Arc<dyn ReminderRepository>,
    notification_provider: Arc<dyn NotificationProvider>,
}

impl ReminderScheduler {
    pub fn new(
        repository: Arc<dyn ReminderRepository>,
        notification_provider: Arc<dyn NotificationProvider>,
    ) -> Self {
        Self {
            repository,
            notification_provider,
        }
    }

    pub fn start(self) {
        let repository = self.repository.clone();
        let notification_provider = self.notification_provider.clone();

        tokio::spawn(async move {
            loop {
                // Poll active reminders (Pending and Scheduled)
                if let Ok(active) = repository.get_active_reminders().await {
                    let now = Utc::now();
                    for reminder in active {
                        // Check if trigger time has arrived
                        if reminder.trigger_at <= now {
                            // Optimistic locking / atomic state transition:
                            // Try to transition Pending -> Triggered or Scheduled -> Triggered
                            let mut claimed = false;
                            for old in &[ReminderStatus::Pending, ReminderStatus::Scheduled] {
                                match repository
                                    .update_status(
                                        &reminder.id,
                                        old.clone(),
                                        ReminderStatus::Triggered,
                                    )
                                    .await
                                {
                                    Ok(true) => {
                                        claimed = true;
                                        break;
                                    }
                                    Err(err) => {
                                        eprintln!(
                                            "[Scheduler] Failed to update reminder status: {:?}",
                                            err
                                        );
                                    }
                                    _ => {}
                                }
                            }

                            if claimed {
                                // Successfully claimed, now dispatch notification
                                if let Err(e) = notification_provider
                                    .send(&reminder.title, reminder.body.as_deref())
                                    .await
                                {
                                    eprintln!("[Scheduler] Failed to send notification for reminder {}: {}", reminder.id, e);
                                }

                                // Handle recurrence
                                if reminder.recurrence != RecurrenceRule::Once {
                                    let next_trigger = match reminder.recurrence {
                                        RecurrenceRule::Daily => {
                                            Some(reminder.trigger_at + chrono::Duration::days(1))
                                        }
                                        RecurrenceRule::Weekly => {
                                            Some(reminder.trigger_at + chrono::Duration::weeks(1))
                                        }
                                        RecurrenceRule::Monthly => {
                                            Some(reminder.trigger_at + chrono::Duration::days(30))
                                        } // Simplified approximation
                                        RecurrenceRule::Once => None,
                                    };

                                    if let Some(next_dt) = next_trigger {
                                        let next_reminder = Reminder {
                                            id: uuid::Uuid::new_v4().to_string(),
                                            entry_id: reminder.entry_id.clone(),
                                            title: reminder.title.clone(),
                                            body: reminder.body.clone(),
                                            trigger_at: next_dt,
                                            recurrence: reminder.recurrence.clone(),
                                            status: ReminderStatus::Pending,
                                            created_at: Utc::now(),
                                            updated_at: Utc::now(),
                                            completed_at: None,
                                        };
                                        let _ = repository.save(&next_reminder).await;
                                    }
                                }
                            }
                        }
                    }
                }

                // Tick interval: 10 seconds for high responsiveness in tests and low resource usage
                sleep(Duration::from_secs(10)).await;
            }
        });
    }
}
