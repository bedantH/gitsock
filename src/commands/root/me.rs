use crate::state::ACCOUNT_STATE;

async fn get_active_account() -> Result<(), Box<dyn std::error::Error>> {
    let state = ACCOUNT_STATE.lock().unwrap();

    if let Some(active_account) = &state.active_account {
        if active_account.username != "".to_string() || active_account.email != "".to_string() {
            println!("🔹 Active Account: ");
            println!("   🧑 Username : {}", active_account.username);
            println!("   📧 Email    : {}\n", active_account.email);
        } else {
            println!(" ⚠️ No Active Account ");
            println!("Run `gitsock account ls && gitsock account switch <USERNAME>`");
        }
    } else {
        println!(" ⚠️ No Active Account ");
    }

    Ok(())
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    get_active_account().await.expect("Error getting active account");

    Ok(())
}