use anyhow::Result;

use super::model::ListenerRecord;

pub fn run_ss() -> Result<Vec<ListenerRecord>> {
    let out = std::process::Command::new("ss")
        .args(["-lptn"])
        .output()?;

    if !out.status.success() {
        return Ok(vec![]);
    }

    let stdout = String::from_utf8_lossy(&out.stdout);
    Ok(parse_ss(&stdout))
}

fn parse_ss(input: &str) -> Vec<ListenerRecord> {
    input
        .lines()
        .skip(1)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 6 {
                return None;
            }
            let local = parts[3];
            let port: u16 = local.rsplit(':').next()?.parse().ok()?;

            let proc_info = parts[5..].join(" ");
            let pid = proc_info
                .split("pid=")
                .nth(1)
                .and_then(|s| s.split(',').next())
                .and_then(|s| s.parse::<i32>().ok())?;

            Some(ListenerRecord {
                port,
                protocol: "tcp".to_string(),
                pid,
            })
        })
        .collect()
}
