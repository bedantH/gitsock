use std::env;
use std::path::Path;

use crate::state::get_active_account;
use crate::{state::get_accounts};
use crate::local_commands::git::{set_email, set_username};

#[cfg(target_os = "windows")]
fn is_valid_path_string(path: &str) -> bool {
    !path.chars().any(|c| "<>:\"\\|?*".contains(c))
}

#[cfg(not(target_os = "windows"))]
fn is_valid_path_string(path: &str) -> bool {
    !path.contains('\0')
}


async fn clone_repo(username_or_alias: Option<String>, url: &str, path: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    match username_or_alias {
        Some(username_or_alias) => {
            let accounts = get_accounts();
            let account = accounts.iter().find(|acc| acc.username == username_or_alias || acc.alias.as_deref() == Some(&username_or_alias));

            if let Some(account) = account {
                if url.starts_with("https://") {
                    eprintln!("Please use SSH URL for cloning with gitsock :_).");
                    return Ok(())
                } else {
                    let parts = url.split('@').collect::<Vec<&str>>();
                    if parts.len() == 2 {
                        let repo_part = parts[1];
                        let modified_url = format!("git@{}:{}", account.alias.as_deref().unwrap_or(&account.username), repo_part.split(':').nth(1).unwrap_or(""));
                        let mut folder_path: Option<String> = url.split('/').nth(1).unwrap().split(".").nth(0).map(|s| { s.to_string() }); // get the default repo name
                        
                        match path.as_ref() {
                            Some(path) => {
                                if is_valid_path_string(path.as_str()) {
                                    folder_path = Some(path.clone());
                                }
                            },
                            None => {}
                        }
                        
                        let status = match path.as_deref() {
                            Some(p) => std::process::Command::new("git")
                                .arg("clone")
                                .arg(if account.default { url } else { modified_url.as_str() })
                                .arg(p)
                                .status()?,
                            None => std::process::Command::new("git")
                                .arg("clone")
                                .arg(if account.default { url } else { modified_url.as_str() })
                                .status()?,
                        };

                        if status.success() {
                            println!("Repository cloned successfully.");
                            
                            if let Some(folder_path) = &folder_path {
                                if Path::new(folder_path).exists() {
                                    env::set_current_dir(folder_path)?;
                                    print!("Changed directory to: {}", folder_path);
                                    
                                    set_username(&account.username, false)?;
                                    set_email(&account.email, false)?;
                                }
                            }
                            
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
            let mut folder_path: Option<String> = url.split('/').nth(1).unwrap().split(".").nth(0).map(|s| s.to_string()); // get the default repo name

            match path {
                Some(path) => {
                    if is_valid_path_string(path.as_str()) {
                        folder_path = Some(path.clone());
                    }
                },
                None => {}
            }

            let status = std::process::Command::new("git")
                .arg("clone")
                .arg(url)
                .status()?;

            if status.success() {
                println!("Repository cloned successfully.");
                
                // check path and change dir
                if let Some(folder_path) = &folder_path {
                    if Path::new(folder_path).exists() {
                        env::set_current_dir(folder_path)?;
                        print!("Changed directory to: {}", folder_path);
                        
                        set_username(&active_account.username, false)?;
                        set_email(&active_account.email, false)?;
                    }
                }
                
            } else {
                eprintln!("Failed to clone the repository.");
                Err("`git clone` failed")?
            }

            Ok(())
        }
    }
}

pub async fn run (username_or_alias: Option<String>, url: String, path: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    clone_repo(username_or_alias, url.as_str(), path).await.expect("Failed to clone repo");
    Ok(())
}