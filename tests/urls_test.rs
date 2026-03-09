use std::fs;
use std::process::Command;

#[test]
fn urls_explains_when_no_services_are_configured() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("config.yml");
    fs::write(&path, "services: {}\n").expect("write config");

    let output = Command::new(env!("CARGO_BIN_EXE_devports"))
        .args(["--config", path.to_str().expect("utf8 path"), "urls"])
        .output()
        .expect("run devports urls");

    assert!(output.status.success(), "command failed: {output:?}");

    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(
        stdout.contains("no configured services; add one with `devports config add <name> --repo <path> --port <port>`"),
        "unexpected stdout: {stdout}"
    );
}
