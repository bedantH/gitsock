use crate::state::ACCOUNT_STATE;

async fn list_all_accounts() -> Result<(), Box<dyn std::error::Error>> {
    let state= ACCOUNT_STATE.lock().unwrap();
    let active_account = state.active_account.clone().unwrap();

    if state.accounts.iter().count() == 0 {
        println!("Oops!, You don't have any GitHub accounts logged in.");
        println!("Run `gitsock account add` to add new account.");

        return Ok(());
    }

    println!("\n==============================");
    println!("   ✅ Authenticated Accounts   ");
    println!("==============================\n");

    for (i, account) in state.accounts.iter().enumerate() {
        println!("🔹 Account #{} {}", i + 1, if account.username == active_account.username { "(Active)" } else { "" });
        println!("   🧑 Username : {}", account.username);
        println!("   📧 Email    : {}\n", account.email);
    }

    Ok(())
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    list_all_accounts().await.expect("Error listing accounts");

    Ok(())
}