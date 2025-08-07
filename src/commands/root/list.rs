use crate::state::ACCOUNT_STATE;

async fn list_all_accounts() -> Result<(), Box<dyn std::error::Error>> {
    let state= ACCOUNT_STATE.lock().unwrap();

    if state.accounts.iter().count() == 0 {
        println!("Oops!, You don't have any GitHub accounts logged in.");
        println!("Run `git log --help` for more information.");

        return Ok(());
    }

    println!("\n==============================");
    println!("   ✅ Authenticated Accounts   ");
    println!("==============================\n");

    for (i, account) in state.accounts.iter().enumerate() {
        println!("🔹 Account #{}", i + 1);
        println!("   🧑 Username : {}", account.username);
        println!("   📧 Email    : {}\n", account.email);
    }

    Ok(())
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    list_all_accounts().await.expect("Error listing accounts");

    Ok(())
}