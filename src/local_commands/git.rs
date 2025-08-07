use std::process::{Command, Stdio};

pub fn set_username(username: String) {
    Command::new("git")
        .args(["config", "--global", "user.name", &format!("{}", username)])
        .output()
        .expect("failed to execute process");
}
pub fn set_email(email: String) {
     Command::new("git")
        .args(["config", "--global", "user.email", &format!("{}", email)])
        .output()
        .expect("failed to execute process");
}