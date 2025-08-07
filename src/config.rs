use std::path::PathBuf;
use serde_json::Value;

lazy_static::lazy_static! {
    pub static ref CONFIG: Value = {
        let config_str = std::fs::read_to_string("config.json")
            .expect("Failed to read config file");
        serde_json::from_str(&config_str).expect("Failed to parse config file")
    };
}

pub fn get_key_as_file(key: &str) -> PathBuf {
    PathBuf::from(CONFIG.get(key).expect("Key not found in config file").as_str().unwrap())
}

pub fn get_config(key: &str) -> Option<&Value> {
    CONFIG.get(key)
}