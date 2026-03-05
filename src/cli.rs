use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "ports", version, about = "Manage local dev service ports")]
pub struct Cli {
    #[arg(long)]
    pub config: Option<PathBuf>,

    #[arg(long)]
    pub json: bool,

    #[arg(long)]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Scan,
    List,
    Tui,
    Kill {
        #[arg(long)]
        port: u16,
        #[arg(long)]
        hard: bool,
        #[arg(long, default_value_t = 1500)]
        timeout_ms: u64,
    },
    Start {
        service: String,
    },
    Doctor,
    Urls {
        #[arg(long)]
        host: Option<String>,
    },
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    Path,
    Init {
        #[arg(long)]
        force: bool,
    },
    Add {
        name: String,
        #[arg(long)]
        repo: PathBuf,
        #[arg(long)]
        port: u16,
        #[arg(long)]
        start: Option<String>,
    },
}
