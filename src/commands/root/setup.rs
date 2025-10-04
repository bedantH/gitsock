use dirs_next as dirs;
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    let file_name = "gitsock.exe";
    #[cfg(unix)]
    let file_name = "gitsock";

    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let gitsock_dir: PathBuf = home_dir.join("gitsock");

    if !gitsock_dir.exists() {
        fs::create_dir_all(&gitsock_dir)?;
    }

    let current_dir = env::current_dir()?;
    let exe_src = current_dir.join(file_name);
    let exe_dest = gitsock_dir.join(file_name);

    if exe_src.exists() {
        fs::copy(&exe_src, &exe_dest)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&exe_dest)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&exe_dest, perms)?;
        }
    } else {
        eprintln!(
            "[WARNING] Executable {} not found in current directory",
            file_name
        );
    }

    let gitsock_str = gitsock_dir.to_string_lossy().to_string();

    #[cfg(unix)]
    {
        let shell = env::var("SHELL").unwrap_or_default();
        let current_path = env::var("PATH").unwrap_or_default();

        let path_separator = ":";
        if !current_path.split(path_separator).any(|p| p == gitsock_str) {
            let new_path = format!("{}:{}", gitsock_str, current_path);
            unsafe {
                env::set_var("PATH", &new_path);
            };
        }

        let export_line = format!(r#"export PATH="{}:$PATH""#, gitsock_str);

        let config_files: Vec<PathBuf> = if shell.contains("zsh") {
            vec![home_dir.join(".zshrc"), home_dir.join(".zshenv")]
        } else if shell.contains("fish") {
            let fish_config_dir = home_dir.join(".config").join("fish");
            if !fish_config_dir.exists() {
                fs::create_dir_all(&fish_config_dir)?;
            }
            let fish_config = fish_config_dir.join("config.fish");
            let fish_export_line = format!(r#"set -gx PATH "{}" $PATH"#, gitsock_str);

            let need_write = if fish_config.exists() {
                !fs::read_to_string(&fish_config)?.contains(&fish_export_line)
            } else {
                true
            };

            if need_write {
                let mut f = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&fish_config)?;
                writeln!(f, "\n{}", fish_export_line)?;
            }

            return Ok(());
        } else {
            vec![
                home_dir.join(".bashrc"),
                home_dir.join(".bash_profile"),
                home_dir.join(".profile"),
            ]
        };

        for config_file in config_files {
            let should_create = config_file
                .file_name()
                .map(|name| name == ".bashrc" || name == ".zshrc")
                .unwrap_or(false);

            if config_file.exists() || should_create {
                let need_write = if config_file.exists() {
                    let content = fs::read_to_string(&config_file)?;
                    !content.contains(&gitsock_str) || !content.contains(&export_line)
                } else {
                    true
                };

                if need_write {
                    let mut f = fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&config_file)?;
                    writeln!(f, "\n# Added by gitsock setup")?;
                    writeln!(f, "{}", export_line)?;
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        let current_path = env::var("PATH").unwrap_or_default();
        let path_separator = ";";

        if !current_path
            .split(path_separator)
            .any(|p| p.to_lowercase() == gitsock_str.to_lowercase())
        {
            let new_path = format!("{};{}", gitsock_str, current_path);
            unsafe {
                env::set_var("PATH", &new_path);
            };
            
            let output = std::process::Command::new("setx")
                .arg("PATH")
                .arg(format!("{};{}", gitsock_str, current_path))
                .output();

            match output {
                Ok(output) if output.status.success() => {
                    println!(
                        "[INFO] Successfully added {} to your permanent PATH",
                        gitsock_str
                    );
                }
                Ok(_) => {
                    eprintln!("[INFO] Add this to your PATH manually: {}", gitsock_str);
                }
                Err(_e) => {
                    eprintln!("[INFO] Add this to your PATH manually: {}", gitsock_str);
                }
            }
        }
    }

    Ok(())
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    setup()?;
    Ok(())
}
