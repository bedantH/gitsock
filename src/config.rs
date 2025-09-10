use std::fs;
use std::path::PathBuf;
use serde_json::Value;

lazy_static::lazy_static! {
    pub static ref CONFIG: Value = {
        let path = "config.json";
        if !std::path::Path::new(path).exists() {
            let default_config = r#"{
                "accounts": ".config/accounts.json",
                "active_account": ".config/active.json",
                "token": ".secret/token.bin",
                "secret": ".secret/secret.bin",
                "ssh_path": "~/.ssh"
            }"#;

            fs::write(path, default_config).expect("Failed to create default config.json");
        }

        let config_str = fs::read_to_string(path)
            .expect("Failed to read config file");

        serde_json::from_str(&config_str).expect("Failed to parse config file")
    };
}

pub fn get_key_as_file(key: &str) -> PathBuf {
    PathBuf::from(CONFIG.get(key).expect("Key not found in config file").as_str().unwrap())
}

pub fn _get_config(key: &str) -> Option<&Value> {
    CONFIG.get(key)
}
