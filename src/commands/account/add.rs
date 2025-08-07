use std::option::Option;
use crate::crypto::encrypt;
use crate::services::{poll_for_token, start_device_login_flow};
use crate::services::{get_user_info};

use crate::state::{update_accounts, update_active_account};
use crate::types::{Account, ActiveAccount};

async fn add_new_account() -> Result<(), Box<dyn std::error::Error>> {
    match start_device_login_flow().await {
        Ok(flow) => {
            println!("🔑 Complete authentication for GitSock from your browser using this code: {}", flow.user_code);
            println!("Didn't open automatically ? Copy the following link in browser and proceed: {}", flow.verification_uri);

            open::that(flow.verification_uri)?;
            let token =poll_for_token(flow.device_code, flow.interval).await?.unwrap();
            let encrypted_token = encrypt(token.as_ref());
            
            match get_user_info(token).await {
                Ok(data) => {
                    println!("Connected to GitSock!, Welcome {:?}", data.login.clone().to_string());
                    
                    update_accounts(|accounts| {
                        let filtered = accounts.iter().filter(|&item| item.username == data.login);

                        if filtered.count() > 0 {
                            println!("Account already exists! Run `gitsock account list` to see all the accounts.");
                        } else {
                            let new_account = Account {
                                email: data.email.clone().expect("Email is None"),
                                name: data.name,
                                username: data.login.clone(),
                                token: Option::from(encrypted_token.clone()),
                                ssh_path: None,
                                alias: None,
                            };

                            accounts.push(new_account);
                        }
                    });

                    update_active_account(|account: &mut ActiveAccount| {
                        if account.username != data.login {
                            account.username = data.login;
                            account.email = data.email.expect("Email is None");
                            account.token = Option::from(encrypted_token);
                        }
                    }).unwrap();
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