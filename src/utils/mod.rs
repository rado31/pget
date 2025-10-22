use std::{path::PathBuf, process::Command};

pub fn get_file_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();

    path.push(".ssh/passwords.toml");
    path
}

pub fn copy_with_wl(password: &str) {
    Command::new("wl-copy").arg(password).status().unwrap();
}
