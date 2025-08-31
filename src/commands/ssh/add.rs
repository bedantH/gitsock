use std::env::home_dir;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use crate::state::{get_accounts, update_accounts};
use crate::utils::{generate_rsa_key_pair, save_key};

pub fn ssh_key_path(alias: &str) -> PathBuf {
    let mut path = home_dir().expect("Failed to get home directory");
    path.push(".ssh");

    let clean_alias = alias.trim();

    let safe_alias: String = clean_alias
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            c if c.is_control() => '_',
            _ => c,
        })
        .collect();

    path.push(format!("github_{}", safe_alias));
    path
}

pub fn ssh_config_path() -> PathBuf {
    let mut path = home_dir().expect("Failed to get home directory");
    path.push(".ssh");
    path.push("config");
    path
}

pub async fn add_ssh_for_account(username_or_alias: &str, default: bool) -> Result<(), Box<dyn std::error::Error>> {
    let accounts = get_accounts();
    let account_data = accounts.iter().find(|&account| {
        account.username == username_or_alias || account.alias.as_deref() == Some(username_or_alias)
    });

    if let Some(account) = account_data.cloned() {
        let alias = account.alias.clone().unwrap();

        let private_key_path = ssh_key_path(alias.as_str());
        let public_key_path = ssh_key_path(&format!("{}.pub", alias));

        println!("Generating SSH Keys...");
        if !private_key_path.exists() && !public_key_path.exists() {
            match generate_rsa_key_pair() {
                Ok((private_key, public_key)) => {
                    fs::create_dir_all(private_key_path.parent().unwrap())?;
                    fs::create_dir_all(public_key_path.parent().unwrap())?;

                    save_key(private_key_path.to_str().unwrap(), &private_key);
                    save_key(public_key_path.to_str().unwrap(), &public_key);

                    // --- 🔑 Step 1: Write SSH config entry BEFORE testing ---
                    let config_path = ssh_config_path();
                    let host = if default { "github.com" } else { alias.as_str() };
                    let config_entry = format!(
                        "\n# GitHub account: {} ({})\nHost {}\n    HostName github.com\n    User git\n    IdentityFile ~/.ssh/{}\n    IdentitiesOnly yes\n\n",
                        account.username,
                        alias,
                        host,
                        private_key_path.file_name().unwrap().to_string_lossy(),
                    );

                    if !config_path.exists() {
                        fs::create_dir_all(config_path.parent().unwrap())?;
                        fs::File::create(&config_path)?;
                    }

                    let config_content = fs::read_to_string(&config_path)?;
                    if !config_content.contains(&format!("Host {}", alias)) {
                        let mut file = OpenOptions::new().append(true).open(&config_path)?;
                        file.write_all(config_entry.as_bytes())?;
                        println!("Added SSH config entry for alias '{}'", alias);
                    } else {
                        println!("SSH config entry for alias '{}' already exists", alias);
                    }

                    // --- 🔑 Step 2: Show public key & manual GitHub steps ---
                    println!("\n==== Public Key (copy this to GitHub SSH Keys) ====\n");
                    println!("{}", public_key);
                    println!("=================================================\n");
                    println!("1. Go to https://github.com/settings/keys");
                    println!("2. Click 'New SSH Key'");
                    println!("3. Paste the above public key and save it.");
                    println!("4. After adding the key, press ENTER to continue...");

                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;

                    loop {
                        println!("Testing SSH connection for alias '{}'", alias);
                        let output = Command::new("ssh")
                            .arg("-T")
                            .arg(alias.as_str())
                            .output()?;

                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);

                        if stdout.contains("successfully authenticated") || stderr.contains("successfully authenticated") {
                            println!("✅ Successfully authenticated with GitHub using alias '{}'", alias);
                            break;
                        } else {
                            println!("❌ Authentication failed.");
                            println!("ssh output:\n{}\n{}", stdout, stderr);
                            println!("Please make sure you have added the public key above to GitHub.");
                            println!("Once done, press ENTER to retry...");

                            let mut input = String::new();
                            io::stdin().read_line(&mut input)?;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to generate keys: {}", e);
                    return Err(e.into());
                }
            }
        } else {
            println!("SSH already exists for this account: {}", alias);
            return Ok(());
        }

        update_accounts(|accounts| {
            if let Some(acc) = accounts.iter_mut().find(|acc| acc.username == account.username) {
                acc.ssh_path = Some(private_key_path.to_string_lossy().to_string());
            }
        });

        Ok(())
    } else {
        eprintln!("Account not found! Please run `gitsock ls` to see a list of integrated accounts");
        Err(Box::from("Account not found"))
    }
}

pub async fn run(username_or_alias: String, default: bool) -> Result<(), Box<dyn std::error::Error>> {
    add_ssh_for_account(username_or_alias.as_str(), default).await?;
    Ok(())
}