use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use anyhow::Result;

use crate::scan::model::ScanRecord;

use super::ps;

pub fn kill_record(record: &ScanRecord, timeout: Duration, hard: bool) -> Result<()> {
    if hard {
        if let Some(pgid) = record.pgid {
            signal_group(pgid, true)?;
            return Ok(());
        }
        signal_pid(record.pid, true)?;
        return Ok(());
    }

    if let Some(pgid) = record.pgid {
        signal_group(pgid, false)?;
        let deadline = Instant::now() + timeout;
        loop {
            if ps::pids_in_group(pgid)?.is_empty() {
                return Ok(());
            }
            if Instant::now() >= deadline {
                signal_group(pgid, true)?;
                return Ok(());
            }
            thread::sleep(Duration::from_millis(100));
        }
    }

    let mut tree = ps::process_tree(record.pid)?;
    tree.sort_unstable();
    for pid in tree.iter().rev() {
        signal_pid(*pid, false)?;
    }
    thread::sleep(timeout);
    for pid in tree.into_iter().rev() {
        signal_pid(pid, true)?;
    }
    Ok(())
}

fn signal_group(pgid: i32, hard: bool) -> Result<()> {
    let sig = if hard { "-KILL" } else { "-TERM" };
    let _ = Command::new("kill")
        .arg(sig)
        .arg(format!("-{pgid}"))
        .status()?;
    Ok(())
}

fn signal_pid(pid: i32, hard: bool) -> Result<()> {
    let sig = if hard { "-KILL" } else { "-TERM" };
    let _ = Command::new("kill").arg(sig).arg(pid.to_string()).status()?;
    Ok(())
}
