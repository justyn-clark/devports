pub mod app;
pub mod keys;
pub mod ui;

use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use crossterm::execute;
use crossterm::event::{self, Event};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use crate::config::Config;
use crate::proc;
use crate::scan;

use self::app::App;
use self::keys::Action;

pub fn run_tui(config_path: &Path, cfg: Config) -> Result<()> {
    let mut app = App::new(join_with_config(&cfg)?);

    let _guard = TerminalGuard::enter()?;
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        if !event::poll(Duration::from_millis(200))? {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            match keys::map_key(key, app.mode, app.show_help) {
                Action::Quit => break,
                Action::Down => app.move_down(),
                Action::Up => app.move_up(),
                Action::PageDown => app.page_down(),
                Action::PageUp => app.page_up(),
                Action::Home => app.move_home(),
                Action::End => app.move_end(),
                Action::Rescan => match join_with_config(&cfg) {
                    Ok(rows) => {
                        app.set_rows(rows);
                        app.set_status("Listener inventory refreshed.");
                    }
                    Err(err) => app.set_status(format!("Rescan failed: {err}")),
                },
                Action::SoftKill => {
                    if let Some(row) = app.selected().cloned() {
                        match proc::kill::kill_record(
                            &row.record,
                            Duration::from_millis(1500),
                            false,
                        ) {
                            Ok(()) => match join_with_config(&cfg) {
                                Ok(rows) => {
                                    app.set_rows(rows);
                                    app.set_status(format!(
                                        "Sent graceful termination to port {} (pid {}).",
                                        row.record.port, row.record.pid
                                    ));
                                }
                                Err(err) => app
                                    .set_status(format!("Process terminated, but refresh failed: {err}")),
                            },
                            Err(err) => app.set_status(format!(
                                "Failed to terminate port {}: {err}",
                                row.record.port
                            )),
                        }
                    } else {
                        app.set_status("No listener selected.");
                    }
                }
                Action::HardKill => {
                    if let Some(row) = app.selected().cloned() {
                        match proc::kill::kill_record(
                            &row.record,
                            Duration::from_millis(1),
                            true,
                        ) {
                            Ok(()) => match join_with_config(&cfg) {
                                Ok(rows) => {
                                    app.set_rows(rows);
                                    app.set_status(format!(
                                        "Force killed port {} (pid {}).",
                                        row.record.port, row.record.pid
                                    ));
                                }
                                Err(err) => app
                                    .set_status(format!("Process killed, but refresh failed: {err}")),
                            },
                            Err(err) => app.set_status(format!(
                                "Failed to force kill port {}: {err}",
                                row.record.port
                            )),
                        }
                    } else {
                        app.set_status("No listener selected.");
                    }
                }
                Action::Start => {
                    if let Some(row) = app.selected().cloned()
                        && let Some(name) = &row.service_name
                        && let Some(svc) = cfg.services.get(name)
                    {
                        match crate::start_configured_service(svc) {
                            Ok(()) => match join_with_config(&cfg) {
                                Ok(rows) => {
                                    app.set_rows(rows);
                                    app.set_status(format!("Started service '{name}'."));
                                }
                                Err(err) => app
                                    .set_status(format!("Service started, but refresh failed: {err}")),
                            },
                            Err(err) => app
                                .set_status(format!("Failed to start service '{name}': {err}")),
                        }
                    } else {
                        app.set_status("Selected row is not a configured service.");
                    }
                }
                Action::OpenConfig => match crate::open_config(config_path) {
                    Ok(()) => app.set_status(format!("Opened config {}", config_path.display())),
                    Err(err) => app.set_status(format!("Failed to open config: {err}")),
                },
                Action::OpenService => {
                    if let Some(row) = app.selected().cloned() {
                        match crate::open_service_url(row.record.port) {
                            Ok(url) => app.set_status(format!("Opened {url}")),
                            Err(err) => app.set_status(format!(
                                "Failed to open selected service on port {}: {err}",
                                row.record.port
                            )),
                        }
                    } else {
                        app.set_status("No listener selected.");
                    }
                }
                Action::StartFilter => app.start_filter(),
                Action::ToggleHelp => app.toggle_help(),
                Action::Backspace => app.pop_filter_char(),
                Action::Input(ch) => app.push_filter_char(ch),
                Action::Confirm => app.finish_filter(),
                Action::Cancel => app.dismiss_overlay(),
                Action::None => {}
            }
        }
    }

    terminal.show_cursor()?;
    Ok(())
}

fn join_with_config(cfg: &Config) -> Result<Vec<crate::scan::model::JoinedPortRecord>> {
    let scan = scan::scan_listeners()?;
    Ok(scan
        .into_iter()
        .map(|record| {
            let service = cfg.match_service(record.repo_root.as_deref(), record.cwd.as_deref());
            crate::scan::model::JoinedPortRecord {
                service_name: service.map(|(name, _)| name.to_string()),
                configured_port: service.map(|(_, svc)| svc.port),
                record,
            }
        })
        .collect())
}

struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> Result<Self> {
        enable_raw_mode()?;
        execute!(std::io::stdout(), EnterAlternateScreen)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(std::io::stdout(), LeaveAlternateScreen);
    }
}
