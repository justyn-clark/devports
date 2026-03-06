use std::path::PathBuf;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ScanRecord {
    pub port: u16,
    pub protocol: String,
    pub pid: i32,
    pub ppid: Option<i32>,
    pub pgid: Option<i32>,
    pub command: String,
    pub cwd: Option<PathBuf>,
    pub repo_root: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize)]
pub struct JoinedPortRecord {
    pub service_name: Option<String>,
    pub configured_port: Option<u16>,
    pub record: ScanRecord,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListenerRecord {
    pub port: u16,
    pub protocol: String,
    pub pid: i32,
}
