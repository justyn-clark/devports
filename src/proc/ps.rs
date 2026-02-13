use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct PsInfo {
    pub pid: i32,
    pub ppid: i32,
    pub pgid: i32,
    pub command: String,
}

pub fn get_ps_info(pid: i32) -> Result<PsInfo> {
    let out = std::process::Command::new("ps")
        .args(["-o", "pid=,ppid=,pgid=,command=", "-p", &pid.to_string()])
        .output()
        .context("failed to run ps")?;

    let stdout = String::from_utf8_lossy(&out.stdout);
    parse_ps_line(stdout.trim()).context("failed to parse ps output")
}

pub fn parse_ps_line(line: &str) -> Result<PsInfo> {
    let mut parts = line.split_whitespace();
    let pid: i32 = parts.next().context("missing pid")?.parse()?;
    let ppid: i32 = parts.next().context("missing ppid")?.parse()?;
    let pgid: i32 = parts.next().context("missing pgid")?.parse()?;
    let command = parts.collect::<Vec<_>>().join(" ");

    Ok(PsInfo {
        pid,
        ppid,
        pgid,
        command,
    })
}

pub fn pids_in_group(pgid: i32) -> Result<Vec<i32>> {
    let out = std::process::Command::new("ps")
        .args(["-o", "pid=", "-g", &pgid.to_string()])
        .output()?;
    if !out.status.success() {
        return Ok(vec![]);
    }

    let stdout = String::from_utf8_lossy(&out.stdout);
    Ok(stdout
        .lines()
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .collect())
}

pub fn process_tree(root_pid: i32) -> Result<Vec<i32>> {
    let out = std::process::Command::new("ps")
        .args(["-e", "-o", "pid=,ppid="])
        .output()?;

    let stdout = String::from_utf8_lossy(&out.stdout);
    let pairs: Vec<(i32, i32)> = stdout
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let pid = parts.next()?.parse().ok()?;
            let ppid = parts.next()?.parse().ok()?;
            Some((pid, ppid))
        })
        .collect();

    let mut stack = vec![root_pid];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(pid) = stack.pop() {
        if !seen.insert(pid) {
            continue;
        }
        for (child, parent) in &pairs {
            if *parent == pid {
                stack.push(*child);
            }
        }
    }

    Ok(seen.into_iter().collect())
}
