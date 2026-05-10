use crate::state::{get_accounts, update_account};
use crate::utils::{generate_rsa_key_pair, save_key};
use dirs_next as dirs;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

pub fn ssh_key_path(alias: &str) -> PathBuf {
    let mut path = dirs::home_dir().expect("Failed to get home directory");
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
    let mut path = dirs::home_dir().expect("Failed to get home directory");
    path.push(".ssh");
    path.push("config");
    path
}

pub async fn add_ssh_for_account(
    username_or_alias: &str,
    default: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let accounts = get_accounts();
    let account_data = accounts.iter().find(|&account| {
        account.username == username_or_alias
            || account.alias.as_deref() == Some(&username_or_alias)
    });

    if let Some(account) = account_data.cloned() {
        let alias = match account.alias.clone() {
            Some(a) => a,
            None => {
                eprintln!("Account '{}' has no alias. Re-add the account and set an alias when prompted.", username_or_alias);
                return Err(Box::from("SSH setup requires an account alias."));
            }
        };

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

                    let config_path = ssh_config_path();

                    if default {
                        let default_already_exists = accounts.iter().find(|a| a.default);

                        if default_already_exists.is_some() {
                            println!("Default account already exists, Aborting!");
                            return Err(Box::from("Default SSH entry already exists."));
                        }
                    }

                    let host = if default {
                        "github.com"
                    } else {
                        alias.as_str()
                    };
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
                            .arg(if default {
                                "github.com"
                            } else {
                                alias.as_str()
                            })
                            .output()?;

                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);

                        if stdout.contains("successfully authenticated")
                            || stderr.contains("successfully authenticated")
                        {
                            println!(
                                "✅ Successfully authenticated with GitHub using alias '{}'",
                                alias
                            );
                            break;
                        } else {
                            println!("❌ Authentication failed.");
                            println!("ssh output:\n{}\n{}", stdout, stderr);
                            println!(
                                "Please make sure you have added the public key above to GitHub."
                            );
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

        update_account(&account.username, |acc| {
            acc.ssh_path = Some(private_key_path.to_string_lossy().to_string());
            acc.default = default;
        });

        Ok(())
    } else {
        eprintln!(
            "Account not found! Please run `gitsock ls` to see a list of integrated accounts"
        );
        Err(Box::from("Account not found"))
    }
}

pub async fn run(
    username_or_alias: String,
    default: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    add_ssh_for_account(username_or_alias.as_str(), default).await?;
    Ok(())
}

pub fn remove_ssh_for_account(account: &crate::types::Account) -> Result<(), Box<dyn std::error::Error>> {
    let alias = match &account.alias {
        Some(a) => a.clone(),
        None => return Ok(()),
    };

    let private_key_path = ssh_key_path(&alias);
    let public_key_path = ssh_key_path(&format!("{}.pub", alias));

    if private_key_path.exists() {
        fs::remove_file(&private_key_path)?;
        println!("Removed SSH private key: {}", private_key_path.display());
    }
    if public_key_path.exists() {
        fs::remove_file(&public_key_path)?;
        println!("Removed SSH public key: {}", public_key_path.display());
    }

    let config_path = ssh_config_path();
    if config_path.exists() {
        remove_from_ssh_config(&config_path, &account.username, &alias)?;
    }

    Ok(())
}

fn remove_from_ssh_config(
    config_path: &PathBuf,
    username: &str,
    alias: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(config_path)?;
    let marker = format!("# GitHub account: {} ({})", username, alias);

    if !content.contains(&marker) {
        return Ok(());
    }

    let lines: Vec<&str> = content.lines().collect();
    let mut output: Vec<&str> = Vec::new();
    let mut in_block = false;
    let mut seen_host = false;

    for line in &lines {
        if line.trim() == marker {
            in_block = true;
            seen_host = false;
            // Drop the blank line we prepended when writing this entry
            while output.last().map(|l: &&str| l.trim().is_empty()).unwrap_or(false) {
                output.pop();
            }
            continue;
        }

        if in_block {
            if !seen_host {
                if line.starts_with("Host ") {
                    seen_host = true;
                }
                continue; // skip blank lines before Host, and the Host line itself
            }
            // After Host line: skip indented options and blank separators
            if line.starts_with("    ") || line.starts_with('\t') || line.trim().is_empty() {
                continue;
            }
            // First non-indented, non-blank line — block is over
            in_block = false;
        }

        output.push(line);
    }

    let result = output.join("\n").trim_end().to_string();
    if result.is_empty() {
        fs::write(config_path, b"")?;
    } else {
        fs::write(config_path, format!("{}\n", result))?;
    }

    println!("Removed SSH config entry for alias '{}'", alias);
    Ok(())
}
