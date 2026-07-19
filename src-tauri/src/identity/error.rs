#[derive(Debug)]
pub enum IdentityError {
    UserExists,
    UserNotFound,
    InvalidPassword,
    InvalidRecoveryKey,
    Storage(String),
}

impl std::fmt::Display for IdentityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for IdentityError {}
