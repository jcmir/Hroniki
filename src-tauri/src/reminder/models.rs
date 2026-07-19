use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReminderStatus {
    Pending,
    Scheduled,
    Triggered,
    Completed,
    Cancelled,
}

impl ReminderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReminderStatus::Pending => "Pending",
            ReminderStatus::Scheduled => "Scheduled",
            ReminderStatus::Triggered => "Triggered",
            ReminderStatus::Completed => "Completed",
            ReminderStatus::Cancelled => "Cancelled",
        }
    }

    pub fn parse(s: &str) -> Self {
        match s {
            "Scheduled" => ReminderStatus::Scheduled,
            "Triggered" => ReminderStatus::Triggered,
            "Completed" => ReminderStatus::Completed,
            "Cancelled" => ReminderStatus::Cancelled,
            _ => ReminderStatus::Pending,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecurrenceRule {
    Once,
    Daily,
    Weekly,
    Monthly,
}

impl RecurrenceRule {
    pub fn as_str(&self) -> &'static str {
        match self {
            RecurrenceRule::Once => "Once",
            RecurrenceRule::Daily => "Daily",
            RecurrenceRule::Weekly => "Weekly",
            RecurrenceRule::Monthly => "Monthly",
        }
    }

    pub fn parse(s: &str) -> Self {
        match s {
            "Daily" => RecurrenceRule::Daily,
            "Weekly" => RecurrenceRule::Weekly,
            "Monthly" => RecurrenceRule::Monthly,
            _ => RecurrenceRule::Once,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub id: String,
    pub entry_id: Option<String>,
    pub title: String,
    pub body: Option<String>,
    pub trigger_at: DateTime<Utc>,
    pub recurrence: RecurrenceRule,
    pub status: ReminderStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}
