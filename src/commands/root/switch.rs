use crate::state::{update_active_account, ACCOUNT_STATE};

async fn switch_account(username_or_alias: &str) -> Result<(), Box<dyn std::error::Error>> {
    let state = ACCOUNT_STATE.lock().unwrap();

    let currently_active = state.active_account.as_ref().cloned().unwrap();

    if currently_active.username == username_or_alias || currently_active.alias == Option::from(username_or_alias.to_string()) {
        println!("Account is already active.");
        return Ok(());
    }

    let all_accounts = state.accounts.clone();

    drop(state);

    if let Some(account) = all_accounts.iter().find(|&account| account.username == username_or_alias) {
        update_active_account(|active_account| {
            active_account.username = account.clone().username;
            active_account.token = account.clone().token;
            active_account.email = account.clone().email;
            active_account.alias = account.clone().alias;
        });

        println!("Welcome Back {:?}!", username_or_alias);
    } else {
        eprintln!("Error: Account does not exist.");
    }

    Ok(())
}

pub async fn run(username: String) -> Result<(), Box<dyn std::error::Error>> {
    switch_account(username.as_str()).await.expect("Unable to switch account, Please contact support.");

    Ok(())
}