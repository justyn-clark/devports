use std::path::PathBuf;

use clap::{Parser, Subcommand};

const DEVPORTS_ASCII: &str = r#"██████╗ ███████╗██╗   ██╗██████╗  ██████╗ ██████╗ ████████╗███████╗
██╔══██╗██╔════╝██║   ██║██╔══██╗██╔═══██╗██╔══██╗╚══██╔══╝██╔════╝
██║  ██║█████╗  ██║   ██║██████╔╝██║   ██║██████╔╝   ██║   ███████╗
██║  ██║██╔══╝  ╚██╗ ██╔╝██╔═══╝ ██║   ██║██╔══██╗   ██║   ╚════██║
██████╔╝███████╗ ╚████╔╝ ██║     ╚██████╔╝██║  ██║   ██║   ███████║
╚═════╝ ╚══════╝  ╚═══╝  ╚═╝      ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚══════╝"#;

const ROOT_EXAMPLES: &str = r#"Examples:
  devports config init
  devports config add web --repo ~/projects/web --port 3000 --start "vite --host 0.0.0.0 --port 3000"
  devports list
  devports doctor --json
  devports tui"#;

#[derive(Parser, Debug)]
#[command(
    name = "devports",
    version,
    about = "Local development port manager",
    long_about = "Track configured local services, inspect live listeners, and act on local development ports from one CLI.",
    before_help = DEVPORTS_ASCII,
    after_help = ROOT_EXAMPLES,
    propagate_version = true
)]
pub struct Cli {
    #[arg(
        long,
        value_name = "PATH",
        global = true,
        help = "Read config from an explicit path instead of ~/.devports/config.yml"
    )]
    pub config: Option<PathBuf>,

    #[arg(
        long,
        global = true,
        help = "Emit machine-readable JSON for commands that support it"
    )]
    pub json: bool,

    #[arg(long, global = true, help = "Disable ANSI color in rendered output")]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(
        about = "Scan active listeners and print raw ownership data",
        long_about = "Inspect active TCP listeners and print the raw listener records discovered from system tools.",
        after_help = "Examples:\n  devports scan\n  devports scan --json"
    )]
    Scan,
    #[command(
        about = "List running listeners with config matches",
        long_about = "Join discovered listeners with configured services and show whether each listener is mapped, unmapped, or mismatched.",
        after_help = "Examples:\n  devports list\n  devports list --json\n  devports --config ./devports.yml list"
    )]
    List,
    #[command(
        about = "Open the interactive terminal interface",
        long_about = "Launch the full-screen terminal UI for browsing listeners, filtering results, starting services, killing processes, and opening service URLs.",
        after_help = "Examples:\n  devports tui\n\nKey flows:\n  Arrows/Home/End/PageUp/PageDown navigate\n  o opens the selected service URL\n  / filters the table, ? shows the full keymap"
    )]
    Tui,
    #[command(
        about = "Terminate the process listening on a port",
        long_about = "Terminate the owning process group for the listener on a specific port, escalating to SIGKILL when requested.",
        after_help = "Examples:\n  devports kill --port 3000\n  devports kill --port 3000 --hard\n  devports kill --port 3000 --timeout-ms 2500"
    )]
    Kill {
        #[arg(long, help = "Port to terminate")]
        port: u16,
        #[arg(long, help = "Skip graceful termination and force kill immediately")]
        hard: bool,
        #[arg(
            long,
            default_value_t = 1500,
            help = "Grace period before escalating from TERM to KILL"
        )]
        timeout_ms: u64,
    },
    #[command(
        about = "Start a configured service",
        long_about = "Start a configured service from its repository root after verifying that the configured port is available.",
        after_help = "Examples:\n  devports start web"
    )]
    Start {
        #[arg(help = "Configured service name")]
        service: String,
    },
    #[command(
        about = "Validate config and runtime state",
        long_about = "Check the configured service registry for missing repos, missing start commands, duplicate ports, and unmapped runtime listeners.",
        after_help = "Examples:\n  devports doctor\n  devports doctor --json\n\nExit status:\n  Returns non-zero when the report contains errors."
    )]
    Doctor,
    #[command(
        about = "Print LAN-friendly URLs for configured services",
        long_about = "Render a URL table for each configured service using the detected hostname or a supplied host value.",
        after_help = "Examples:\n  devports urls\n  devports urls --host devbox.local"
    )]
    Urls {
        #[arg(long, help = "Hostname or IP to place in generated URLs")]
        host: Option<String>,
    },
    #[command(
        about = "Open a configured service in the browser",
        long_about = "Resolve a configured service to its expected local URL and open it with the platform browser launcher.",
        after_help = "Examples:\n  devports open web"
    )]
    Open {
        #[arg(help = "Configured service name")]
        name: String,
    },
    #[command(
        about = "Inspect and edit config state",
        long_about = "Resolve the config path, initialize a config file, or add services to the registry.",
        after_help = "Examples:\n  devports config path\n  devports config init\n  devports config add api --repo ~/projects/api --port 4000 --start \"pnpm dev\""
    )]
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    #[command(
        about = "Print the active config path",
        after_help = "Example:\n  devports config path"
    )]
    Path,
    #[command(
        about = "Create an empty config file",
        after_help = "Examples:\n  devports config init\n  devports config init --force"
    )]
    Init {
        #[arg(long, help = "Overwrite an existing config file")]
        force: bool,
    },
    #[command(
        about = "Add or update a configured service",
        long_about = "Insert or update a named service in the registry with its repository root, expected port, and optional start command.",
        after_help = "Examples:\n  devports config add web --repo ~/projects/web --port 3000 --start \"vite --host 0.0.0.0 --port 3000\"\n  devports config add api --repo ~/projects/api --port 4000"
    )]
    Add {
        #[arg(help = "Service name to create or update")]
        name: String,
        #[arg(long, value_name = "PATH", help = "Repository root for the service")]
        repo: PathBuf,
        #[arg(long, help = "Expected local listener port")]
        port: u16,
        #[arg(long, help = "Command used by `devports start <service>`")]
        start: Option<String>,
    },
}
