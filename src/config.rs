use std::fs;
use std::path::PathBuf;
use serde_json::{json, Value};
use dirs_next as dirs;

lazy_static::lazy_static! {
    pub static ref CONFIG: Value = {
        #[cfg(debug_assertions)]
        let path = "config.json";

        #[cfg(not(debug_assertions))]
        let path = expand_home("~/gitsock/config.json");

        if !std::path::Path::new(&path).exists() {
            #[cfg(debug_assertions)]
            let default_config = json!({
                "accounts": ".config/accounts.json",
                "active_account": ".config/active.json",
                "token": ".secret/token.bin",
                "secret": ".secret/secret.bin",
                "ssh_path": ".ssh"
            });

            #[cfg(not(debug_assertions))]
            let default_config = json!({
                "accounts": expand_home("~/gitsock/.config/accounts.json"),
                "active_account": expand_home("~/gitsock/.config/active.json"),
                "token": expand_home("~/gitsock/.secret/token.bin"),
                "secret": expand_home("~/gitsock/.secret/secret.bin"),
                "ssh_path": expand_home("~/.ssh")
            });

            fs::write(&path, serde_json::to_string_pretty(&default_config).unwrap())
                .expect("Failed to create default config.json");
        }

        let config_str = fs::read_to_string(&path)
            .expect("Failed to read config file");

        serde_json::from_str(&config_str).expect("Failed to parse config file")
    };
}

pub fn get_key_as_file(key: &str) -> PathBuf {
    PathBuf::from(
        CONFIG.get(key)
            .expect("Key not found in config file")
            .as_str()
            .unwrap()
    )
}

pub fn _get_config(key: &str) -> Option<&Value> {
    CONFIG.get(key)
}

/// Expand `~` into absolute home path
fn expand_home(path: &str) -> String {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(stripped).to_string_lossy().into_owned();
        }
    }
    path.to_string()
}
