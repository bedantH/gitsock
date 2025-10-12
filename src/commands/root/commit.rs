use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;

use crate::local_commands::git::{get_local_git_config, set_email, set_username};
use crate::state::{get_accounts, get_active_account};

fn pattern_percentages(patterns: &[&str], text: &str) -> Option<Vec<(String, f64)>> {
    let mut map = HashMap::new();

    if text.is_empty() || patterns.is_empty() {
        return None;
    }

    for &pattern in patterns {
        if pattern.is_empty() {
            continue;
        }

        let mut count = 0;
        let mut start = 0;

        while let Some(pos) = text[start..].find(pattern) {
            count += 1;
            start += pos + pattern.len();
        }

        if count > 0 {
            map.insert(pattern.to_string(), count as f64);
        }
    }

    if map.is_empty() {
        return None;
    }

    let max = map.values().cloned().fold(f64::MIN, f64::max);
    let results: Vec<(String, f64)> = map
        .into_iter()
        .filter(|(_, v)| (v - max).abs() < 1e-6)
        .collect();

    Some(results)
}

fn get_commit_message(msg: Option<String>) -> io::Result<String> {
    match msg {
        Some(m) => Ok(m),
        None => {
            print!("Enter commit message: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            Ok(input.trim().to_string())
        }
    }
}

fn run_commit(msg: &str) -> io::Result<()> {
    let status = Command::new("git").args(["commit", "-m", msg]).status()?;
    if !status.success() {
        eprintln!("Failed to commit!");
    }
    Ok(())
}

async fn commit(msg: Option<String>, username_or_alias: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let accounts = get_accounts();
    let active_account = get_active_account();

    let status = Command::new("git").args(["rev-parse", "--is-inside-work-tree"]).output()?;
    if !status.status.success() || String::from_utf8_lossy(&status.stdout).trim() != "true" {
        return Err(Box::from("Not a git repository!"));
    }

    if let Some(username_or_alias) = username_or_alias {
        if let Some(account) = accounts.iter().find(|&account| account.username == username_or_alias || account.alias.as_deref() == Some(&username_or_alias)) {
            set_username(&account.username, false)?;
            set_email(&account.email, false)?;

            let commit_msg = get_commit_message(msg)?;
            run_commit(&commit_msg)?;

            return Ok(());
        } else {
            eprintln!("Error: Account does not exist.");
            return Ok(());            
        }
    } else {
        if let Some((name, email)) = get_local_git_config() {
            println!("Local config found, using: {} <{}>", name, email);
            let commit_msg = get_commit_message(msg)?;
            run_commit(&commit_msg)?;
            return Ok(());
        }

        let output = Command::new("git").arg("log").output()?;
        if !output.status.success() {
            eprintln!("No commits found, falling back to active account.");
            set_username(&active_account.username, false)?;
            set_email(&active_account.email, false)?;
            let commit_msg = get_commit_message(msg)?;
            run_commit(&commit_msg)?;
            return Ok(());
        }

        let logs = String::from_utf8_lossy(&output.stdout);
        let list_of_names = accounts.iter().map(|acc| acc.username.as_str()).collect::<Vec<&str>>();

        let match_account = if let Some(matches) = pattern_percentages(&list_of_names, &logs) {
            if matches.len() == 1 {
                matches[0].0.clone()
            } else {
                println!("Multiple accounts matched:");
                for (i, (name, _)) in matches.iter().enumerate() {
                    println!("  [{}] {}", i + 1, name);
                }

                print!("Select an account to use [1-{}]: ", matches.len());
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let choice: usize = input.trim().parse().unwrap_or(0);

                if choice == 0 || choice > matches.len() {
                    return Err(Box::from("Invalid selection"));
                }
                matches[choice - 1].0.clone()
            }
        } else {
            active_account.username.clone()
        };

        if match_account != active_account.username {
            if let Some(matched) = accounts.iter().find(|acc| acc.username == match_account) {
                println!("Setting account {:?} for this repository", matched.username);
                set_username(&matched.username, false)?;
                set_email(&matched.email, false)?;
            }
        } else {
            set_username(&active_account.username, false)?;
            set_email(&active_account.email, false)?;
        }

        let commit_msg = get_commit_message(msg)?;
        run_commit(&commit_msg)?;
        Ok(())
    }
}

pub async fn run(msg: Option<String>, username_or_alias: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    commit(msg, username_or_alias).await
}
