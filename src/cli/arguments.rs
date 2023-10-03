//1 Cli parser
use clap::Parser;

/// only accepts the target path
#[derive(Parser, Debug)]
#[command(about)]
pub struct Cli {
    /// leet code project's path
    #[arg(short, long)]
    pub path: Option<String>,

    /// language.json directory
    #[arg(short, long)]
    pub language_path: Option<String>,
}

pub fn arguments() -> Cli {
    Cli::parse()
}
