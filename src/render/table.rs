use crate::config::doctor::DoctorReport;
use crate::scan::model::{JoinedPortRecord, ScanRecord};

pub fn print_list_table(records: &[JoinedPortRecord]) {
    println!(
        "{:<6}  {:<20}  {:<8}  {:<8}  {:<8}  {}",
        "PORT", "SERVICE", "PID", "PGID", "MATCH", "COMMAND"
    );
    for row in records {
        let match_state = match (row.service_name.as_ref(), row.configured_port) {
            (Some(_), Some(expected)) if expected == row.record.port => "ok",
            (Some(_), Some(_)) => "mismatch",
            _ => "unmapped",
        };
        println!(
            "{:<6}  {:<20}  {:<8}  {:<8}  {:<8}  {}",
            row.record.port,
            row.service_name.as_deref().unwrap_or("-"),
            row.record.pid,
            row.record
                .pgid
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
            match_state,
            row.record.command
        );
    }
}

pub fn print_doctor_report(report: &DoctorReport) {
    if report.issues.is_empty() {
        println!("doctor: ok");
        return;
    }
    for issue in &report.issues {
        println!("[{}] {}: {}", issue.level, issue.code, issue.message);
    }
}

pub fn print_conflict(record: &ScanRecord) {
    eprintln!(
        "port conflict: port={} pid={} pgid={} cmd={} cwd={} repo_root={}",
        record.port,
        record.pid,
        record
            .pgid
            .map(|v| v.to_string())
            .unwrap_or_else(|| "-".to_string()),
        record.command,
        record
            .cwd
            .as_deref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "-".to_string()),
        record
            .repo_root
            .as_deref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "-".to_string()),
    );
}
