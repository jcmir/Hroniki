use super::models::{RecurrenceRule, ReminderStatus};
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
                // Poll active reminders (Pending, Scheduled, Failed)
                match repository.get_active_reminders().await {
                    Ok(active) => {
                        let now = Utc::now();
                        for mut reminder in active {
                            if reminder.trigger_at <= now {
                                // Claim reminder (atomic transition to Triggered)
                                let mut claimed = false;
                                for old in &[
                                    ReminderStatus::Pending,
                                    ReminderStatus::Scheduled,
                                    ReminderStatus::Failed,
                                ] {
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
                                            tracing::error!(id = ?reminder.id, error = ?err, "Failed to update reminder status to Triggered");
                                        }
                                        _ => {}
                                    }
                                }

                                if claimed {
                                    // Try sending notification
                                    match notification_provider
                                        .send(&reminder.title, reminder.body.as_deref())
                                        .await
                                    {
                                        Ok(_) => {
                                            // Handle recurrence: atomically reschedule current reminder by updating trigger_at and status -> Pending
                                            if reminder.recurrence != RecurrenceRule::Once {
                                                let next_trigger = match reminder.recurrence {
                                                    RecurrenceRule::Daily => Some(
                                                        reminder.trigger_at
                                                            + chrono::Duration::days(1),
                                                    ),
                                                    RecurrenceRule::Weekly => Some(
                                                        reminder.trigger_at
                                                            + chrono::Duration::weeks(1),
                                                    ),
                                                    RecurrenceRule::Monthly => Some(
                                                        reminder.trigger_at
                                                            + chrono::Duration::days(30),
                                                    ),
                                                    RecurrenceRule::EveryNDays(n) => Some(
                                                        reminder.trigger_at
                                                            + chrono::Duration::days(n),
                                                    ),
                                                    RecurrenceRule::Once => None,
                                                };

                                                if let Some(next_dt) = next_trigger {
                                                    reminder.trigger_at = next_dt;
                                                    reminder.status = ReminderStatus::Pending;
                                                    reminder.updated_at = Utc::now();

                                                    if let Err(err) =
                                                        repository.save(&reminder).await
                                                    {
                                                        tracing::error!(id = ?reminder.id, error = ?err, "Failed to reschedule recurring reminder");
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            tracing::error!(id = ?reminder.id, error = ?e, "Failed to send notification. Rolling back status to Failed");
                                            // Rollback state back to Failed so it is eligible for retry
                                            if let Err(err) = repository
                                                .update_status(
                                                    &reminder.id,
                                                    ReminderStatus::Triggered,
                                                    ReminderStatus::Failed,
                                                )
                                                .await
                                            {
                                                tracing::error!(id = ?reminder.id, error = ?err, "Critical: Failed to rollback status to Failed after notification delivery failure");
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => {
                        tracing::error!(error = ?err, "Scheduler failed to poll active reminders from repository");
                    }
                }

                sleep(Duration::from_secs(10)).await;
            }
        });
    }
}
