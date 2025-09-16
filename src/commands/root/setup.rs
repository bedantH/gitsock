use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use dirs_next as dirs;

pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let gitsock_dir: PathBuf = home_dir.join(".gitsock");
    if !gitsock_dir.exists() {
        fs::create_dir_all(&gitsock_dir)?;
    }

    let current_dir = env::current_dir()?;
    let exe_src = current_dir.join("gitsock.exe");
    let exe_dest = gitsock_dir.join("gitsock.exe");

    if exe_src.exists() {
        fs::copy(&exe_src, &exe_dest)?;
    }

    let mut current_path = env::var("PATH")?;
    let gitsock_str = gitsock_dir.to_string_lossy().to_string();

    if !current_path.contains(&gitsock_str) {
        current_path = format!("{}:{}", gitsock_str, current_path);
        unsafe { env::set_var("PATH", &current_path) };
    }

    #[cfg(unix)]
    {
        let shell = env::var("SHELL").unwrap_or_default();
        let config_file = if shell.contains("zsh") {
            home_dir.join(".zshrc")
        } else {
            home_dir.join(".bashrc")
        };

        let export_line = r#"export PATH="$HOME/.gitsock:$PATH""#;

        if fs::read_to_string(&config_file).unwrap_or_default().contains(export_line) == false {
            let mut f = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&config_file)?;
            writeln!(f, "\n{}", export_line)?;
        }
    }

    #[cfg(target_os = "windows")]
    {
        let output = std::process::Command::new("setx")
            .arg("PATH")
            .arg(format!("{};{}", gitsock_str, env::var("PATH")?))
            .output()?;

        if !output.status.success() {
            eprintln!("Warning: failed to persist PATH with setx");
        }
    }

    Ok(())
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    setup()?;
    Ok(())
}
