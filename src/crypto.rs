use aes_gcm::{aead::{Aead, OsRng}, AeadCore, Aes256Gcm, KeyInit, Nonce};

use crate::state::with_key;

pub fn encrypt(data: &[u8]) -> Vec<u8> {
    with_key(|key| {
        let cipher = Aes256Gcm::new(&key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let mut encrypted = cipher.encrypt(&nonce, data).expect("Failed to encrypt");

        let mut result = nonce.to_vec();
        result.append(&mut encrypted);

        result
    })
}