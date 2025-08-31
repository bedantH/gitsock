use std::process::Command;

pub fn set_username(username: &str, global: bool) -> std::io::Result<()> {
    let args = if global {
        vec!["config", "--global", "user.name", username]
    } else {
        vec!["config", "user.name", username]
    };

    let status = Command::new("git").args(&args).status()?;
    if !status.success() {
        eprintln!("Failed to set git username");
    }
    Ok(())
}

pub fn set_email(email: &str, global: bool) -> std::io::Result<()> {
    let args = if global {
        vec!["config", "--global", "user.email", email]
    } else {
        vec!["config", "user.email", email]
    };

    let status = Command::new("git").args(&args).status()?;
    if !status.success() {
        eprintln!("Failed to set git email");
    }
    Ok(())
}

pub fn get_local_git_config() -> Option<(String, String)> {
    let name = Command::new("git")
        .args(["config", "--get", "user.name"])
        .output()
        .ok()
        .and_then(|o| if o.status.success() { Some(String::from_utf8_lossy(&o.stdout).trim().to_string()) } else { None });

    let email = Command::new("git")
        .args(["config", "--get", "user.email"])
        .output()
        .ok()
        .and_then(|o| if o.status.success() { Some(String::from_utf8_lossy(&o.stdout).trim().to_string()) } else { None });

    if let (Some(n), Some(e)) = (name, email) {
        Some((n, e))
    } else {
        None
    }
}
