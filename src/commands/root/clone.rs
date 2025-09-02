use crate::state::get_active_account;
use crate::{state::get_accounts};
use crate::local_commands::git::{set_email, set_username};


async fn clone_repo(username_or_alias: Option<String>, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    match username_or_alias {
        Some(username_or_alias) => {
            let accounts = get_accounts();
            let account = accounts.iter().find(|acc| acc.username == username_or_alias || acc.alias.as_deref() == Some(&username_or_alias));

            if let Some(account) = account {
                set_username(&account.username, false)?;
                set_email(&account.email, false)?;

                if url.starts_with("https://") {
                    eprintln!("Please use SSH URL for cloning with multiple accounts.");
                    return Ok(())
                } else {
                    let parts = url.split('@').collect::<Vec<&str>>();
                    if parts.len() == 2 {
                        let repo_part = parts[1];
                        let modified_url = format!("git@{}:{}", account.alias.as_deref().unwrap_or(&account.username), repo_part.split(':').nth(1).unwrap_or(""));
                        
                        let status = std::process::Command::new("git")
                            .arg("clone")
                            .arg(modified_url)
                            .status()?;

                        if status.success() {
                            println!("Repository cloned successfully.");
                            return Ok(());
                        } else {
                            eprintln!("Failed to clone the repository.");
                            Err("`git clone` failed")?
                        }
                    } else {
                        eprintln!("Incorrect URL! Please try again with a correct URL.");
                        return Ok(())
                    }
                }
            } else {
                return Err(Box::from(format!("Account with username or alias '{}' not found", username_or_alias)));
            }
        },
        None => {
            let active_account = get_active_account();
            set_username(&active_account.username, false)?;
            set_email(&active_account.email, false)?;

            let status = std::process::Command::new("git")
                .arg("clone")
                .arg(url)
                .status()?;

            if status.success() {
                println!("Repository cloned successfully.");
            } else {
                eprintln!("Failed to clone the repository.");
                Err("`git clone` failed")?
            }

            Ok(())
        }
    }
}

pub async fn run (username_or_alias: Option<String>, url: String) -> Result<(), Box<dyn std::error::Error>> {
    clone_repo(username_or_alias, url.as_str()).await.expect("Failed to clone repo");
    Ok(())
}