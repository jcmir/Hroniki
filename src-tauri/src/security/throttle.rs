use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthThrottleState {
    pub failed_attempts: u32,
    pub last_failure: Option<DateTime<Utc>>,
    pub locked_until: Option<DateTime<Utc>>,
}

impl Default for AuthThrottleState {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthThrottleState {
    pub fn new() -> Self {
        Self {
            failed_attempts: 0,
            last_failure: None,
            locked_until: None,
        }
    }

    pub fn is_locked_out(&self, now: DateTime<Utc>) -> (bool, u64) {
        if let Some(until) = self.locked_until {
            if now < until {
                let remaining = (until - now).num_seconds().max(0) as u64;
                return (true, remaining);
            }
        }
        (false, 0)
    }

    pub fn register_failure(
        &mut self,
        now: DateTime<Utc>,
        max_attempts: u32,
        lockout_secs: i64,
    ) -> bool {
        self.failed_attempts += 1;
        self.last_failure = Some(now);

        if self.failed_attempts >= max_attempts {
            self.locked_until = Some(now + chrono::Duration::seconds(lockout_secs));
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.failed_attempts = 0;
        self.last_failure = None;
        self.locked_until = None;
    }
}
