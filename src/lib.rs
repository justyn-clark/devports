pub mod cli;
pub mod config;
pub mod proc;
pub mod render;
pub mod scan;
pub mod tui;

use std::path::Path;
use std::process::Command;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use cli::{Commands, ConfigCommands};
use config::{Config, ServiceConfig};
use scan::model::{JoinedPortRecord, ScanRecord};

pub fn execute(cli: cli::Cli) -> Result<()> {
    let config_path = cli
        .config
        .clone()
        .unwrap_or_else(config::default_config_path);

    match cli.command {
        Commands::Scan => {
            let records = scan::scan_listeners()?;
            render::json::print_pretty(&records)?;
        }
        Commands::List => {
            let cfg = Config::load(&config_path)?;
            let joined = join_records(&scan::scan_listeners()?, &cfg);
            if cli.json {
                render::json::print_pretty(&joined)?;
            } else {
                render::table::print_list_table(&joined);
            }
        }
        Commands::Tui => {
            let cfg = Config::load(&config_path)?;
            tui::run_tui(config_path.as_path(), cfg)?;
        }
        Commands::Kill {
            port,
            hard,
            timeout_ms,
        } => {
            let records = scan::scan_listeners()?;
            let record = records.iter().find(|r| r.port == port).cloned();
            let Some(record) = record else {
                bail!("no listener found for port {port}");
            };
            proc::kill::kill_record(&record, Duration::from_millis(timeout_ms), hard)?;
            eprintln!("killed listener on port {port}");
        }
        Commands::Start { service } => {
            let cfg = Config::load(&config_path)?;
            let svc = cfg.service(&service)?;
            ensure_port_available(svc.port)?;
            start_service(svc)?;
        }
        Commands::Doctor => {
            let cfg = Config::load(&config_path)?;
            let report = config::doctor::doctor(&cfg, &scan::scan_listeners()?);
            if cli.json {
                render::json::print_pretty(&report)?;
            } else {
                render::table::print_doctor_report(&report);
            }
            if report.has_errors() {
                std::process::exit(1);
            }
        }
        Commands::Urls { host } => {
            let cfg = Config::load(&config_path)?;
            let records = scan::scan_listeners()?;

            let hostname = host.unwrap_or_else(|| {
                hostname::get()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            });

            let mut services = cfg.services.iter().collect::<Vec<_>>();
            services.sort_by(|(a, _), (b, _)| a.cmp(b));

            for (name, svc) in services {
                let running = records.iter().find(|r| r.port == svc.port);
                let status = if running.is_some() { "LISTEN" } else { "DOWN" };
                println!("{:<20} http://{}:{} {}", name, hostname, svc.port, status);
            }
        }
        Commands::Config { command } => match command {
            ConfigCommands::Path => {
                println!("{}", config_path.display());
            }
            ConfigCommands::Init { force } => {
                config::init_config(&config_path, force)?;
                println!("created {}", config_path.display());
            }
            ConfigCommands::Add {
                name,
                repo,
                port,
                start,
            } => {
                config::add_service(&config_path, name, repo, port, start)?;
                println!("service added");
            }
        },
    }

    Ok(())
}

fn join_records(records: &[ScanRecord], cfg: &Config) -> Vec<JoinedPortRecord> {
    records
        .iter()
        .cloned()
        .map(|record| {
            let service = cfg.match_service(record.repo_root.as_deref(), record.cwd.as_deref());
            JoinedPortRecord {
                service_name: service.map(|(name, _)| name.to_string()),
                configured_port: service.map(|(_, svc)| svc.port),
                record,
            }
        })
        .collect()
}

fn ensure_port_available(port: u16) -> Result<()> {
    let records = scan::scan_listeners()?;
    if let Some(existing) = records.into_iter().find(|r| r.port == port) {
        render::table::print_conflict(&existing);
        bail!("port {port} is already in use");
    }
    Ok(())
}

fn start_service(service: &ServiceConfig) -> Result<()> {
    let start = service
        .start
        .as_deref()
        .context("service is missing start command")?;

    if !service.repo.exists() {
        bail!("repo does not exist: {}", service.repo.display());
    }

    let mut cmd = Command::new("zsh");
    cmd.arg("-lc").arg(start).current_dir(&service.repo);
    let status = cmd.status().context("failed to execute start command")?;
    if !status.success() {
        bail!("start command exited with {status}");
    }

    Ok(())
}

pub fn open_config(path: &Path) -> Result<()> {
    let status = if cfg!(target_os = "macos") {
        Command::new("open").arg(path).status()
    } else {
        Command::new("xdg-open").arg(path).status()
    }
    .context("failed to open config")?;

    if !status.success() {
        bail!("open config command failed");
    }

    Ok(())
}
