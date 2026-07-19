use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionVerifier {
    pub version: u32,
    pub salt: String,
    pub nonce: String,
    pub ciphertext: Vec<u8>,
    pub tag: Vec<u8>,
}

impl SessionVerifier {
    pub fn new(salt: String, nonce: String, ciphertext: Vec<u8>, tag: Vec<u8>) -> Self {
        Self {
            version: 1,
            salt,
            nonce,
            ciphertext,
            tag,
        }
    }
}
