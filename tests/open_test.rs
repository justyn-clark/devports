use std::fs;

use devports::{
    cli::{Cli, Commands},
    execute,
};

#[test]
fn open_reports_missing_service_from_config() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("config.yml");
    fs::write(&path, "services: {}\n").expect("write config");

    let err = execute(Cli {
        config: Some(path),
        json: false,
        no_color: false,
        command: Some(Commands::Open {
            name: "web".to_string(),
        }),
    })
    .expect_err("missing service should error");

    assert!(
        err.to_string().contains("service not found in config: web"),
        "unexpected error: {err:?}"
    );
}
