use std::path::PathBuf;

use anyhow::{Context, Result};

pub fn get_cwd(pid: i32) -> Result<Option<PathBuf>> {
    if cfg!(target_os = "macos") {
        return get_cwd_macos(pid);
    }
    get_cwd_linux(pid)
}

fn get_cwd_macos(pid: i32) -> Result<Option<PathBuf>> {
    let out = std::process::Command::new("lsof")
        .args(["-p", &pid.to_string()])
        .output()
        .context("failed to run lsof for cwd")?;

    if !out.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&out.stdout);
    for line in stdout.lines() {
        if line.contains(" cwd ") {
            let path = line
                .split_whitespace()
                .last()
                .map(PathBuf::from)
                .filter(|p| p.is_absolute());
            return Ok(path);
        }
    }

    Ok(None)
}

fn get_cwd_linux(pid: i32) -> Result<Option<PathBuf>> {
    let link = format!("/proc/{pid}/cwd");
    match std::fs::read_link(&link) {
        Ok(path) => Ok(Some(path)),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err).context("failed reading /proc cwd symlink"),
    }
}
