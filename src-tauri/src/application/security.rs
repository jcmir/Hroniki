use pbkdf2::pbkdf2;
use sha2::Sha256;
use pbkdf2::hmac::Hmac;
use rand::RngCore;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce, Key
};

pub fn generate_salt() -> Vec<u8> {
    let mut salt = vec![0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

pub fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn from_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    (0..hex_str.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex_str[i..i + 2], 16)
                .map_err(|e| e.to_string())
        })
        .collect()
}

pub fn hash_pin(pin: &str, salt: &[u8]) -> String {
    let mut hash = [0u8; 32];
    pbkdf2::<Hmac<Sha256>>(pin.as_bytes(), salt, 10_000, &mut hash).unwrap();
    to_hex(&hash)
}

pub fn derive_key(password: &str, salt: &[u8]) -> Key<Aes256Gcm> {
    let mut key_bytes = [0u8; 32];
    pbkdf2::<Hmac<Sha256>>(password.as_bytes(), salt, 10_000, &mut key_bytes).unwrap();
    *Key::<Aes256Gcm>::from_slice(&key_bytes)
}

pub fn encrypt_data(data: &[u8], password: &str) -> Result<Vec<u8>, String> {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);

    let key = derive_key(password, &salt);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let encrypted_bytes = cipher.encrypt(nonce, data).map_err(|e| e.to_string())?;

    let mut result = Vec::with_capacity(16 + 12 + encrypted_bytes.len());
    result.extend_from_slice(&salt);
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&encrypted_bytes);
    Ok(result)
}

pub fn decrypt_data(encrypted_data: &[u8], password: &str) -> Result<Vec<u8>, String> {
    if encrypted_data.len() < 28 {
        return Err("Invalid encrypted data length".to_string());
    }

    let salt = &encrypted_data[0..16];
    let nonce_bytes = &encrypted_data[16..28];
    let ciphertext = &encrypted_data[28..];

    let key = derive_key(password, salt);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher.decrypt(nonce, ciphertext).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pin_hashing() {
        let salt = generate_salt();
        let hash1 = hash_pin("1234", &salt);
        let hash2 = hash_pin("1234", &salt);
        let hash3 = hash_pin("4321", &salt);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_encryption_decryption() {
        let payload = b"Secret chronology backup data";
        let password = "MySecurePassword123";

        let encrypted = encrypt_data(payload, password).unwrap();
        let decrypted = decrypt_data(&encrypted, password).unwrap();

        assert_eq!(payload.to_vec(), decrypted);
    }

    #[test]
    fn test_corrupted_archive_fails_decryption() {
        let payload = b"Secret chronology backup data";
        let password = "MySecurePassword123";
        let mut encrypted = encrypt_data(payload, password).unwrap();
        
        // Corrupt one byte of ciphertext (skip salt 16 bytes + nonce 12 bytes = 28)
        if encrypted.len() > 30 {
            encrypted[30] ^= 0xFF; 
        }

        let result = decrypt_data(&encrypted, password);
        assert!(result.is_err());
    }
}
