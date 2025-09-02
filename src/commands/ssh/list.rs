use crate::state::get_accounts;

async fn list_ssh_accounts() -> Result<(), Box<dyn std::error::Error>> {
    let accounts = get_accounts();
    let accounts_with_ssh = accounts.iter().filter(|&account| account.ssh_path != None).collect::<Vec<_>>();

    // separators
    println!("=========================================================================");
    println!("Note: This will only list all github accounts you have integrated SSH \nfor using gitsock. Any SSH files you have created on your \nown won't show up here.");
    println!("=========================================================================");
    println!("\nAccounts SSH integrated for: ");

    for (index, &account) in accounts_with_ssh.iter().enumerate() {
        println!("  {}. {:?} ({})", index + 1, &account.username, &account.clone().ssh_path.unwrap_or_else( || "N/A".to_string() ) );
    }

    Ok(())
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    list_ssh_accounts().await.expect("Unable to list ssh accounts");
    Ok(())
}