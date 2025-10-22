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
        if let Some(password) = servers.get(&args.name) {
            let password = password.as_str().unwrap_or("");

            if env::var("WAYLAND_DISPLAY").is_ok() {
                utils::copy_with_wl(password);
                return;
            };

            let mut ctx = ClipboardContext::new().unwrap();
            ctx.set_contents(password.to_string()).unwrap();
        }

        println!("Doesn't exist");
        return;
    }

    servers.insert(args.name, Value::String(args.password.unwrap()));
    fs::write(&path, toml::to_string_pretty(&data).unwrap()).unwrap();
}
