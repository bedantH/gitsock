use crate::crypto::decrypt;
use crate::services::remove_account_github;
use crate::state::{update_accounts, update_active_account, ACCOUNT_STATE};
use crate::types::{ActiveAccount};

async fn remove_account(username: String) -> Result<(), Box<dyn std::error::Error>> {
    let (token, is_active) = {
        let state = ACCOUNT_STATE.lock().unwrap();
        let mut token: Vec<u8> = vec![];
        let mut is_active = false;

        if let Some(active_account) = &state.active_account {
            if active_account.username == username {
                token = active_account.clone().token.unwrap();
                is_active = true;
            }
        }

        if let Some(account) = state.accounts.iter().find(|&account| account.username == username) {
            token = account.clone().token.unwrap()
        }

        if token.is_empty() {
            return Err("Account not found!".into());
        }

        (token, is_active)
    };

    let decrypted_token = match String::from_utf8(decrypt(&*token)) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Decrypted data is not valid UTF-8");
            return Err("Err Code: 0001".into());
        }
    };

    match remove_account_github(decrypted_token).await {
        Ok(true) => {
            update_accounts(|accounts| {
                accounts.retain(|item| item.username != username);
            });

            if is_active {
                update_active_account(|active_account| {
                    *active_account = ActiveAccount::default();
                });
            }
            
            println!("Account removed successfully!");
        },
        Ok(false) => {
            eprintln!("Account not found!");
        },
        Err(err) => {
            eprintln!("Failed to remove account github: {err}");
        }
    }

    Ok(())
}

pub async fn run(username: String) -> Result<(), Box<dyn std::error::Error>> {
    remove_account(username).await.expect("Unable to remove account, Please contact support.");

    Ok(())
}