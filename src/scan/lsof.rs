use anyhow::Result;

use super::model::ListenerRecord;

pub fn parse_lsof_listeners(input: &str) -> Vec<ListenerRecord> {
    input
        .lines()
        .skip(1)
        .filter_map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Option<ListenerRecord> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }
    let pid: i32 = parts.get(1)?.parse().ok()?;

    let endpoint = if parts.last()? == &"(LISTEN)" {
        parts.get(parts.len().saturating_sub(2))?
    } else {
        parts.last()?
    };

    let port: u16 = endpoint.rsplit(':').next()?.parse().ok()?;
    let protocol = if line.contains(" TCP") {
        "tcp".to_string()
    } else {
        "unknown".to_string()
    };

    Some(ListenerRecord {
        port,
        protocol,
        pid,
    })
}

pub fn run_lsof() -> Result<Vec<ListenerRecord>> {
    let out = std::process::Command::new("lsof")
        .args(["-nP", "-iTCP", "-sTCP:LISTEN"])
        .output()?;

    if !out.status.success() {
        return Ok(vec![]);
    }

    let stdout = String::from_utf8_lossy(&out.stdout);
    Ok(parse_lsof_listeners(&stdout))
}
