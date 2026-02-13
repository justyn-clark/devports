use std::fs;

use ports::config::Config;

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
