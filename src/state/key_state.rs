use aes_gcm::{
    aead::{KeyInit, OsRng},
    Aes256Gcm, Key,
};
use once_cell::sync::Lazy;
use std::fs;
use std::sync::Mutex;

use crate::config::get_key_as_file;

#[derive(Debug)]
pub struct KeyState {
    key: Key<Aes256Gcm>,
}

pub static KEY_STATE: Lazy<Mutex<KeyState>> = Lazy::new(|| {
    let key = load_or_generate_key();
    Mutex::new(KeyState { key })
});

fn load_or_generate_key() -> Key<Aes256Gcm> {
    let path = get_key_as_file("secret");

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Failed to create key directory");
        }
    }

    if path.exists() {
        let key_bytes = fs::read(&path).expect("Failed to read key file");
        let slice = Key::<Aes256Gcm>::from_slice(&key_bytes);
        slice.clone()
    } else {
        let key = Aes256Gcm::generate_key(&mut OsRng);
        fs::write(&path, key.as_slice()).expect("Failed to write key file");
        key
    }
}

pub fn with_key<F, R>(f: F) -> R
where
    F: FnOnce(&Key<Aes256Gcm>) -> R,
{
    let state = KEY_STATE.lock().unwrap();
    f(&state.key)
}
