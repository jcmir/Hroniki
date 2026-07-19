use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    System,
    Dark,
    Light,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserSettings {
    pub profile_name: Option<String>,
    pub theme: ThemeMode,
    pub auto_lock_minutes: u32,
    pub notifications: bool,
    pub biometric_unlock: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            profile_name: None,
            theme: ThemeMode::System,
            auto_lock_minutes: 5,
            notifications: true,
            biometric_unlock: false,
        }
    }
}
