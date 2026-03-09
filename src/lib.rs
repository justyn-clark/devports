pub mod cli;
pub mod config;
pub mod proc;
pub mod render;
pub mod scan;
pub mod tui;

use std::fs::{self, OpenOptions};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use cli::{Commands, ConfigCommands};
use config::{Config, ServiceConfig};
use scan::model::{JoinedPortRecord, ScanRecord};

pub(crate) struct StartedService {
    pub pid: u32,
    pub log_path: std::path::PathBuf,
}

pub fn execute(cli: cli::Cli) -> Result<()> {
    let config_path = cli
        .config
        .clone()
        .unwrap_or_else(config::default_config_path);

    let Some(command) = cli.command else {
        return Ok(());
    };

    match command {
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
            let launched = start_configured_service(svc)?;
            println!(
                "launched '{}' at {} (pid {}, log {})",
                service,
                service_url(svc.port),
                launched.pid,
                launched.log_path.display()
            );
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

            let hostname = host.unwrap_or_else(default_host);

            let mut services = cfg.services.iter().collect::<Vec<_>>();
            services.sort_by(|(a, _), (b, _)| a.cmp(b));

            for (name, svc) in services {
                let running = records.iter().find(|r| r.port == svc.port);
                let status = if running.is_some() { "LISTEN" } else { "DOWN" };
                println!("{:<20} http://{}:{} {}", name, hostname, svc.port, status);
            }
        }
        Commands::Open { name } => {
            let cfg = Config::load(&config_path)?;
            let svc = cfg.service(&name)?;
            let is_listening = scan::scan_listeners()?
                .iter()
                .any(|record| record.port == svc.port);
            if !is_listening {
                bail!(
                    "service '{name}' is configured for {} but is not listening",
                    service_url(svc.port)
                );
            }
            let url = service_url(svc.port);

            open_target(&url)?;
            println!("opened {}", url);
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

pub(crate) fn ensure_port_available(port: u16) -> Result<()> {
    let records = scan::scan_listeners()?;
    if let Some(existing) = records.into_iter().find(|r| r.port == port) {
        render::table::print_conflict(&existing);
        bail!("port {port} is already in use");
    }
    Ok(())
}

pub(crate) fn start_configured_service(service: &ServiceConfig) -> Result<StartedService> {
    ensure_port_available(service.port)?;
    start_service(service)
}

pub(crate) fn start_service(service: &ServiceConfig) -> Result<StartedService> {
    let start = service
        .start
        .as_deref()
        .context("service is missing start command")?;

    if !service.repo.exists() {
        bail!("repo does not exist: {}", service.repo.display());
    }

    let log_path = start_log_path(service)?;
    let stdout = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .with_context(|| format!("failed to open start log {}", log_path.display()))?;
    let stderr = stdout
        .try_clone()
        .with_context(|| format!("failed to clone start log handle {}", log_path.display()))?;

    let mut cmd = Command::new("zsh");
    cmd.arg("-lc")
        .arg(start)
        .current_dir(&service.repo)
        .stdin(Stdio::null())
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr));

    let mut child = cmd.spawn().context("failed to launch start command")?;
    let pid = child.id();

    thread::sleep(Duration::from_millis(150));
    if let Some(status) = child
        .try_wait()
        .context("failed while checking launched service")?
        && !status.success()
    {
        bail!(
            "start command exited immediately with {status}; see {}",
            log_path.display()
        );
    }

    Ok(StartedService { pid, log_path })
}

pub(crate) fn default_host() -> String {
    hostname::get()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

pub(crate) fn local_service_url(port: u16) -> String {
    format!("http://127.0.0.1:{port}")
}

pub(crate) fn lan_service_url(port: u16) -> String {
    format!("http://{}:{}", default_host(), port)
}

pub(crate) fn service_url(port: u16) -> String {
    local_service_url(port)
}

pub(crate) fn open_service_url(port: u16) -> Result<String> {
    let url = service_url(port);
    open_target(&url)?;
    Ok(url)
}

fn open_target(target: &str) -> Result<()> {
    let command = if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    };
    let status = Command::new(command)
        .arg(target)
        .status()
        .with_context(|| format!("failed to launch {command}"))?;

    if !status.success() {
        bail!("{command} exited with {status}");
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

fn start_log_path(service: &ServiceConfig) -> Result<std::path::PathBuf> {
    let log_dir = service.repo.join(".devports");
    fs::create_dir_all(&log_dir)
        .with_context(|| format!("failed to create log dir {}", log_dir.display()))?;
    Ok(log_dir.join("start.log"))
}

#[cfg(test)]
mod tests {
    use super::{ServiceConfig, lan_service_url, local_service_url, service_url, start_service};
    use std::fs;
    use std::thread;
    use std::time::{Duration, Instant};

    #[test]
    fn service_url_defaults_to_local_loopback() {
        assert_eq!(service_url(5173), "http://127.0.0.1:5173");
        assert_eq!(local_service_url(3000), "http://127.0.0.1:3000");
    }

    #[test]
    fn lan_service_url_keeps_hostname_urls_available() {
        let url = lan_service_url(4173);

        assert!(url.starts_with("http://"));
        assert!(url.ends_with(":4173"));
    }

    #[test]
    fn start_service_launches_in_background() {
        let dir = tempfile::tempdir().expect("tempdir");
        let repo = dir.path().join("repo");
        fs::create_dir_all(&repo).expect("create repo");
        let output = dir.path().join("started.txt");
        let service = ServiceConfig {
            repo: repo.clone(),
            port: 45555,
            start: Some(format!("sleep 1; echo started > {}", output.display())),
            tags: vec![],
        };

        let started_at = Instant::now();
        let launched = start_service(&service).expect("launch service");

        assert!(
            started_at.elapsed() < Duration::from_millis(900),
            "start should return quickly for background launches"
        );
        assert!(launched.pid > 0);
        assert_eq!(launched.log_path, repo.join(".devports").join("start.log"));

        for _ in 0..30 {
            if output.exists() {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }

        assert!(output.exists(), "background start command did not complete");
    }
}
