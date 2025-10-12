use crate::{local_commands::git::{set_email, set_username}, state::{update_active_account, ACCOUNT_STATE}};

async fn switch_account(username_or_alias: &str, local: bool) -> Result<(), Box<dyn std::error::Error>> {
    let state = ACCOUNT_STATE.lock().unwrap();

    let currently_active = state.active_account.as_ref().cloned().unwrap();

    if currently_active.username == username_or_alias || currently_active.alias.as_deref() == Some(&username_or_alias) {
        println!("Account is already active.");
        return Ok(());
    }

    let all_accounts = state.accounts.clone();

    drop(state);

    if let Some(account) = all_accounts.iter().find(|&account| account.username == username_or_alias || account.alias.as_deref() == Some(&username_or_alias)) {
        if local {
            set_email(&account.email, false)?;
            set_username(&account.username, false)?;

            println!("Switched to account {:?} for this repository", username_or_alias);

            return Ok(());
        } else {
            update_active_account(|active_account| {
                active_account.username = account.clone().username;
                active_account.token = account.clone().token;
                active_account.email = account.clone().email;
                active_account.alias = account.clone().alias;
            });

            set_email(&account.email, true)?;
            set_username(&account.username, true)?;
    
            println!("Welcome Back {:?}!", username_or_alias);
        }
    } else {
        eprintln!("Error: Account does not exist.");
    }

    Ok(())
}

pub async fn run(username: String, local: bool) -> Result<(), Box<dyn std::error::Error>> {
    switch_account(username.as_str(), local).await.expect("Unable to switch account, Please contact support.");

    Ok(())
}