pub mod lsof;
pub mod model;
pub mod ss;

use std::collections::HashSet;

use anyhow::Result;
use model::{ListenerRecord, ScanRecord};

use crate::proc;

pub fn scan_listeners() -> Result<Vec<ScanRecord>> {
    let listeners = discover_listeners()?;
    let mut records = Vec::new();

    for listener in listeners {
        let ps = proc::ps::get_ps_info(listener.pid).ok();
        let cwd = proc::cwd::get_cwd(listener.pid).ok().flatten();
        let repo_root = cwd.as_deref().and_then(proc::repo::resolve_repo_root);

        records.push(ScanRecord {
            port: listener.port,
            protocol: listener.protocol,
            pid: listener.pid,
            ppid: ps.as_ref().map(|v| v.ppid),
            pgid: ps.as_ref().map(|v| v.pgid),
            command: ps
                .map(|v| v.command)
                .unwrap_or_else(|| "<unknown>".to_string()),
            cwd,
            repo_root,
        });
    }

    records.sort_by_key(|r| r.port);
    Ok(records)
}

fn discover_listeners() -> Result<Vec<ListenerRecord>> {
    let listeners = if cfg!(target_os = "macos") {
        lsof::run_lsof()?
    } else {
        match lsof::run_lsof() {
            Ok(records) if !records.is_empty() => records,
            _ => ss::run_ss()?,
        }
    };

    let mut seen = HashSet::new();
    let mut unique = Vec::new();
    for listener in listeners {
        if seen.insert(listener.clone()) {
            unique.push(listener);
        }
    }

    Ok(unique)
}
