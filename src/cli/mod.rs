use clap::Parser;

/// Store and retrieve passwords for servers
#[derive(Parser)]
#[command(name = "pget", version, about, arg_required_else_help = true)]
pub struct Args {
    /// Add or update server password
    #[arg(short = 'a')]
    pub add: bool,

    /// Name
    pub name: String,

    /// Password (only used with -a)
    pub password: Option<String>,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }
}
