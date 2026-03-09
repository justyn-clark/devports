use std::fs;
use std::path::PathBuf;

use devports::config::{Config, doctor};
use devports::scan::model::ScanRecord;

#[test]
fn parse_config_yaml() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("ports.yml");
    fs::write(
        &path,
        r#"services:
  justbeatz-web:
    repo: /tmp/justbeatz-web
    port: 3000
    start: bun run dev
  justbeatz-api:
    repo: /tmp/justbeatz-api
    port: 3001
"#,
    )
    .expect("write config");

    let cfg = Config::load(&path).expect("load config");
    assert_eq!(cfg.services.len(), 2);
    assert_eq!(cfg.services["justbeatz-web"].port, 3000);
    assert_eq!(cfg.services["justbeatz-api"].start, None);
}

#[test]
fn doctor_warns_for_unmapped_local_dev_listener() {
    let cfg = Config {
        services: Default::default(),
    };
    let home = std::env::var("HOME").expect("home");
    let running = vec![ScanRecord {
        port: 4123,
        protocol: "tcp".to_string(),
        pid: 100,
        ppid: Some(1),
        pgid: Some(100),
        command: "vite".to_string(),
        cwd: Some(PathBuf::from(home).join("projects/web")),
        repo_root: None,
    }];

    let report = doctor::doctor(&cfg, &running);

    assert_eq!(report.issues.len(), 1);
    assert_eq!(report.issues[0].code, "unmapped_listener");
}

#[test]
fn doctor_ignores_unmapped_system_listener_noise() {
    let cfg = Config {
        services: Default::default(),
    };
    let running = vec![ScanRecord {
        port: 8080,
        protocol: "tcp".to_string(),
        pid: 200,
        ppid: Some(1),
        pgid: Some(200),
        command: "system-daemon".to_string(),
        cwd: Some(PathBuf::from("/System/Library/CoreServices")),
        repo_root: None,
    }];

    let report = doctor::doctor(&cfg, &running);

    assert!(
        report.issues.is_empty(),
        "unexpected issues: {:?}",
        report.issues
    );
}
