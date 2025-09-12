use std::fs;
use std::fs::File;
use std::io::{Read};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::types::{Account, ActiveAccount};
use crate::config::get_key_as_file;
use crate::local_commands::git::{set_email, set_username};

#[derive(Debug)]
pub struct AccountState {
    pub(crate) accounts: Vec<Account>,
    pub(crate) active_account: Option<ActiveAccount>,
}

pub static ACCOUNT_STATE: Lazy<Mutex<AccountState>> = Lazy::new(|| Mutex::new(AccountState {
    accounts: load_or_generate_accounts_file(),
    active_account: load_or_generate_active_account_file(),
}));

fn load_or_generate_accounts_file() -> Vec<Account> {
    let path = get_key_as_file("accounts");

    if !path.exists() {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).expect("Failed to create key directory");
            }
        }
    }

    if path.exists() {
        let file = File::open(&path).expect("No accounts file");
        let accounts: Vec<Account> = serde_json::from_reader(file).unwrap();

        accounts
    } else {
        fs::write(path, "[]").unwrap();
        vec![]
    }
}

fn load_or_generate_active_account_file() -> Option<ActiveAccount> {
    let path = get_key_as_file("active_account");

    if !path.exists() {
        if let Some(parent) = path.parent() {
            let msg = format!("Unable to create config directory: {}", parent.display());
            fs::create_dir_all(parent).expect(&msg);
        }
    }

    if path.exists() && path.metadata().unwrap().len() > 0 {
        let mut file = File::open(&path).expect("No active account file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Error reading from file");

        if !contents.is_empty() {
            let active_account: ActiveAccount = serde_json::from_slice(&contents).unwrap();
            Some(active_account)
        } else {
            Some(ActiveAccount::default())
        }
    } else {
        fs::write(path, []).unwrap();
        Some(ActiveAccount::default())
    }
}

pub fn get_accounts() -> Vec<Account> {
    let state = ACCOUNT_STATE.lock().unwrap();
    state.accounts.clone()
}

pub fn get_active_account() -> ActiveAccount {
    let state = ACCOUNT_STATE.lock().unwrap();
    state.active_account.clone().unwrap()
}

pub fn update_account(
    username: &str,
    updater: impl FnOnce(&mut Account),
) -> Option<Account> {
    let mut state = ACCOUNT_STATE.lock().unwrap();
    let updated: Option<Account>;

    {
        if let Some(existing) = state.accounts.iter_mut().find(|a| a.username == username) {
            updater(existing);
            updated = Some(existing.clone());
        } else {
            return None;
        }
    } // <- mutable borrow of state.accounts ends here

    // now safe to serialize/write
    let accounts_path = get_key_as_file("accounts");
    let json = serde_json::to_string_pretty(&state.accounts).unwrap();
    fs::write(accounts_path, json).expect("Error writing to accounts file");

    updated
}

pub fn update_accounts<F>(f: F)
where F: FnOnce(&mut Vec<Account>),
{
    let mut state = ACCOUNT_STATE.lock().unwrap();
    f(&mut state.accounts);

    let accounts_path = get_key_as_file("accounts");
    let json = serde_json::to_string_pretty(&state.accounts).unwrap();

    fs::write(&accounts_path, &json).expect("Error writing to accounts file");
}

pub fn update_active_account<F>(f: F) -> Option<ActiveAccount>
where
    F: FnOnce(&mut ActiveAccount),
{
    let mut state = ACCOUNT_STATE.lock().unwrap();

    if let Some(active_account) = state.active_account.as_mut() {
        f(active_account);
        
        set_email(&*active_account.clone().email, true).expect("Setting email globally failed");
        set_username(&*active_account.clone().username, true).expect("Setting username failed");
        
        let active_account_path = get_key_as_file("active_account");
        let json = serde_json::to_string_pretty(&active_account).unwrap();

        fs::write(active_account_path, json).expect("Error writing to active account file");
    }

    state.active_account.clone()
}
