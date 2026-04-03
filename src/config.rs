use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub services: BTreeMap<String, ServiceConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServiceConfig {
    pub repo: PathBuf,
    pub port: u16,
    pub start: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let text = fs::read_to_string(path)
            .with_context(|| format!("failed to read config {}", path.display()))?;
        let cfg: Self = serde_yaml::from_str(&text)
            .with_context(|| format!("failed to parse config {}", path.display()))?;
        Ok(cfg)
    }

    pub fn service(&self, name: &str) -> Result<&ServiceConfig> {
        self.services
            .get(name)
            .with_context(|| format!("service not found in config: {name}"))
    }

    pub fn match_service<'a>(
        &'a self,
        repo_root: Option<&Path>,
        cwd: Option<&Path>,
    ) -> Option<(&'a str, &'a ServiceConfig)> {
        let candidate = repo_root.or(cwd)?;
        self.services
            .iter()
            .filter(|(_, svc)| path_prefix_eq(&svc.repo, candidate))
            .max_by_key(|(_, svc)| svc.repo.components().count())
            .map(|(name, svc)| (name.as_str(), svc))
    }
}

fn path_prefix_eq(prefix: &Path, full: &Path) -> bool {
    full.starts_with(prefix)
}

pub fn default_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".jcn").join("ports.yml")
}

pub fn init_config(path: &Path, force: bool) -> Result<()> {
    if path.exists() && !force {
        bail!("config already exists (use --force to overwrite)");
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, "services: {}\n")?;

    Ok(())
}

pub fn add_service(
    path: &Path,
    name: String,
    repo: PathBuf,
    port: u16,
    start: Option<String>,
) -> Result<()> {
    let mut cfg = if path.exists() {
        Config::load(path)?
    } else {
        Config {
            services: Default::default(),
        }
    };

    cfg.services.insert(
        name,
        ServiceConfig {
            repo,
            port,
            start,
            tags: vec![],
        },
    );

    let yaml = serde_yaml::to_string(&cfg)?;
    fs::write(path, yaml)?;

    Ok(())
}

pub mod doctor {
    use super::*;
    use crate::scan::model::ScanRecord;

    #[derive(Debug, Clone, Serialize)]
    pub struct DoctorIssue {
        pub level: String,
        pub code: String,
        pub message: String,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct DoctorReport {
        pub issues: Vec<DoctorIssue>,
    }

    impl DoctorReport {
        pub fn has_errors(&self) -> bool {
            self.issues.iter().any(|i| i.level == "error")
        }
    }

    pub fn doctor(cfg: &Config, running: &[ScanRecord]) -> DoctorReport {
        let mut issues = Vec::new();

        let mut ports = HashMap::<u16, Vec<&str>>::new();
        for (name, svc) in &cfg.services {
            ports.entry(svc.port).or_default().push(name.as_str());
            if !svc.repo.exists() {
                issues.push(DoctorIssue {
                    level: "error".to_string(),
                    code: "missing_repo".to_string(),
                    message: format!("service {name} repo missing: {}", svc.repo.display()),
                });
            }
            if svc.start.as_deref().unwrap_or("").trim().is_empty() {
                issues.push(DoctorIssue {
                    level: "warn".to_string(),
                    code: "missing_start".to_string(),
                    message: format!("service {name} has no start command"),
                });
            }
        }

        for (port, owners) in ports {
            if owners.len() > 1 {
                issues.push(DoctorIssue {
                    level: "error".to_string(),
                    code: "duplicate_port".to_string(),
                    message: format!("port {port} is assigned to: {}", owners.join(", ")),
                });
            }
        }

        let configured_ports: HashSet<u16> = cfg.services.values().map(|s| s.port).collect();
        for record in running {
            if configured_ports.contains(&record.port) {
                continue;
            }
            issues.push(DoctorIssue {
                level: "warn".to_string(),
                code: "unmapped_listener".to_string(),
                message: format!(
                    "listening port {} pid {} not mapped in config",
                    record.port, record.pid
                ),
            });
        }

        DoctorReport { issues }
    }
}
