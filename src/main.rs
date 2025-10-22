use std::{env, fs};

use copypasta::{ClipboardContext, ClipboardProvider};
use toml::Value;

mod cli;
mod utils;

fn main() {
    let args = cli::Args::new();

    let path = utils::get_file_path();

    if !path.exists() {
        fs::write(&path, "[servers]\n").unwrap();
        return;
    }

    let content = fs::read_to_string(&path).unwrap();
    let mut data: Value = toml::from_str(&content).unwrap();

    let servers = data
        .get_mut("servers")
        .and_then(Value::as_table_mut)
        .expect("[servers] section missing");

    if !args.add {
        match servers.get(&args.name) {
            Some(password) => {
                if env::var("WAYLAND_DISPLAY").is_ok() {
                    utils::copy_with_wl(password.as_str().unwrap_or(""));
                    return;
                };

                let mut ctx = ClipboardContext::new().unwrap();
                let pass = password.as_str().unwrap_or("").to_string();
                ctx.set_contents(pass).unwrap();
            }
            None => println!("There is no password for this server"),
        };

        return;
    }

    let val = Value::String(args.password.unwrap());

    servers.insert(args.name, val);

    let new_toml = toml::to_string_pretty(&data).unwrap();
    fs::write(&path, new_toml).unwrap();
}
