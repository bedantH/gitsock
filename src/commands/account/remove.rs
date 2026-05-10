use crate::commands::ssh::remove_ssh_for_account;
use crate::config::get_key_as_file;
use crate::state::{get_accounts, get_active_account, update_accounts};
use std::fs;

async fn remove_account(username: String) -> Result<(), Box<dyn std::error::Error>> {
    let accounts = get_accounts();

    let account = accounts.iter().find(|a| a.username == username).cloned()
        .ok_or_else(|| format!("Account '{}' not found. Run `gitsock ls` to see all accounts.", username))?;

    let was_active = get_active_account().username == username;

    // Clean up SSH keys and config entry before removing from state
    if let Err(e) = remove_ssh_for_account(&account) {
        eprintln!("Warning: could not fully clean up SSH files: {}", e);
    }

    update_accounts(|accounts| {
        accounts.retain(|a| a.username != username);
    });

    if was_active {
        let active_path = get_key_as_file("active_account");
        fs::write(active_path, b"")?;
        println!("Note: '{}' was the active account. Run `gitsock use <username>` to switch to another.", username);
    }

    println!("Account '{}' removed successfully.", username);
    Ok(())
}

pub async fn run(username: String) -> Result<(), Box<dyn std::error::Error>> {
    remove_account(username).await
}