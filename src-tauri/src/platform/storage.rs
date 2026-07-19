use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecretKind {
    DatabaseKey,
    SessionToken,
    RecoveryKey,
    PinVerifier,
}

impl SecretKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            SecretKind::DatabaseKey => "db_key",
            SecretKind::SessionToken => "session_token",
            SecretKind::RecoveryKey => "recovery_key",
            SecretKind::PinVerifier => "pin_verifier",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretIdentifier {
    pub kind: SecretKind,
    pub namespace: String,
}

#[async_trait]
pub trait SecureStoragePlatform: Send + Sync {
    async fn store(&self, id: SecretIdentifier, value: &[u8]) -> Result<(), String>;
    async fn load(&self, id: SecretIdentifier) -> Result<Option<Vec<u8>>, String>;
    async fn delete(&self, id: SecretIdentifier) -> Result<(), String>;
}
