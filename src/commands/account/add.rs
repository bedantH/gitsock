use crate::crypto::encrypt;
use crate::services::{poll_for_token, start_device_login_flow};
use crate::services::{get_user_info};

use crate::state::{update_accounts, update_active_account};
use crate::types::{Account, ActiveAccount};
use std::io::{self, Write};

async fn add_new_account() -> Result<(), Box<dyn std::error::Error>> {
    match start_device_login_flow().await {
        Ok(flow) => {
            println!("🔑 Complete authentication for GitSock from your browser using this code: {}", flow.user_code);
            println!("Didn't open automatically ? Copy the following link in browser and proceed: {}", flow.verification_uri);

            open::that(flow.verification_uri)?;
            let token = poll_for_token(flow.device_code, flow.interval).await?.unwrap();
            let encrypted_token = encrypt(token.as_ref());

            match get_user_info(token).await {
                Ok(data) => {
                    let username = if data.login.is_empty() {
                        println!("Could not fetch your GitHub username automatically.");
                        print!("Please enter your GitHub username: ");
                        io::stdout().flush().unwrap();
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();
                        input.trim().to_string()
                    } else {
                        data.login.clone()
                    };

                    let email = match data.email {
                        Some(ref e) if !e.is_empty() => e.clone(),
                        _ => {
                            println!("Could not fetch your email automatically (your GitHub profile email may be set to private).");
                            print!("Please enter your email address: ");
                            io::stdout().flush().unwrap();
                            let mut input = String::new();
                            io::stdin().read_line(&mut input).unwrap(); // <- this blocks till entered input
                            input.trim().to_string()
                        }
                    };

                    println!("Connected to GitSock! Welcome, {}!", username);

                    // Prompt for alias BEFORE updating accounts
                    print!("What alias would you like to set for this account? (Press Enter to skip): ");
                    io::stdout().flush().unwrap();

                    let mut alias_input = String::new();
                    io::stdin().read_line(&mut alias_input).unwrap();
                    let alias_input = alias_input.trim();
                    let alias = if alias_input.is_empty() {
                        None
                    } else {
                        Some(alias_input.to_string())
                    };

                    // Prepare new account
                    let new_account = Account {
                        email: email.clone(),
                        name: username.clone(),
                        username: username.clone(),
                        token: Some(encrypted_token.clone()),
                        ssh_path: None,
                        alias,
                        default: false,
                    };

                    let mut is_new_account = false;
                    update_accounts(|accounts| {
                        let exists = accounts.iter().any(|item| item.username == username);
                        if exists {
                            println!("Account already exists! Run `gitsock account list` to see all the accounts.");
                        } else {
                            accounts.push(new_account.clone());
                            is_new_account = true;
                        }
                    });

                    if is_new_account {
                        update_active_account(|account: &mut ActiveAccount| {
                            if account.username != new_account.username {
                                account.username = new_account.username.clone();
                                account.email = new_account.email.clone();
                                account.token = Some(encrypted_token);
                                account.alias = new_account.alias.clone();
                            }
                        }).unwrap();
                    }
                },
                Err(e) => {
                    eprintln!("Failed to get user info: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to start device flow: {}", e);
        }
    }

    Ok(())
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    add_new_account().await.expect("Adding new account failed!");

    Ok(())
}
