use crate::state::ACCOUNT_STATE;

async fn remove_account(username: String) -> Result<(), Box<dyn std::error::Error>> {
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

    println!("Account removed successfully!");

    Ok(())
}

pub async fn run(username: String) -> Result<(), Box<dyn std::error::Error>> {
    remove_account(username).await.expect("Unable to remove account, Please contact support.");

    Ok(())
}