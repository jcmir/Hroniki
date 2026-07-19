use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReminderStatus {
    Pending,
    Scheduled,
    Triggered,
    Completed,
    Cancelled,
    Failed,
}

impl ReminderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReminderStatus::Pending => "Pending",
            ReminderStatus::Scheduled => "Scheduled",
            ReminderStatus::Triggered => "Triggered",
            ReminderStatus::Completed => "Completed",
            ReminderStatus::Cancelled => "Cancelled",
            ReminderStatus::Failed => "Failed",
        }
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "Pending" => Ok(ReminderStatus::Pending),
            "Scheduled" => Ok(ReminderStatus::Scheduled),
            "Triggered" => Ok(ReminderStatus::Triggered),
            "Completed" => Ok(ReminderStatus::Completed),
            "Cancelled" => Ok(ReminderStatus::Cancelled),
            "Failed" => Ok(ReminderStatus::Failed),
            _ => Err(format!("Unknown reminder status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecurrenceRule {
    Once,
    Daily,
    Weekly,
    Monthly,
    EveryNDays(i64),
}

impl RecurrenceRule {
    pub fn as_str(&self) -> String {
        match self {
            RecurrenceRule::Once => "Once".to_string(),
            RecurrenceRule::Daily => "Daily".to_string(),
            RecurrenceRule::Weekly => "Weekly".to_string(),
            RecurrenceRule::Monthly => "Monthly".to_string(),
            RecurrenceRule::EveryNDays(n) => format!("EveryNDays:{}", n),
        }
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "Once" => Ok(RecurrenceRule::Once),
            "Daily" => Ok(RecurrenceRule::Daily),
            "Weekly" => Ok(RecurrenceRule::Weekly),
            "Monthly" => Ok(RecurrenceRule::Monthly),
            _ if s.starts_with("EveryNDays:") => {
                let parts: Vec<&str> = s.split(':').collect();
                if parts.len() == 2 {
                    if let Ok(n) = parts[1].parse::<i64>() {
                        return Ok(RecurrenceRule::EveryNDays(n));
                    }
                }
                Err(format!("Invalid EveryNDays format: {}", s))
            }
            _ => Err(format!("Unknown recurrence rule: {}", s)),
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
