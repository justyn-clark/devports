pub mod app;
pub mod keys;
pub mod ui;

use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use crate::config::Config;
use crate::proc;
use crate::scan;

use self::app::App;
use self::keys::Action;

pub fn run_tui(config_path: &Path, cfg: Config) -> Result<()> {
    let mut app = App::new(join_with_config(&cfg)?);

    enable_raw_mode()?;
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if !event::poll(Duration::from_millis(200))? {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            match keys::map_key(key) {
                Action::Quit => break,
                Action::Down => app.move_down(),
                Action::Up => app.move_up(),
                Action::Rescan => app.set_rows(join_with_config(&cfg)?),
                Action::Term => {
                    if let Some(row) = app.selected() {
                        let _ = proc::kill::kill_record(
                            &row.record,
                            Duration::from_millis(1500),
                            false,
                        );
                        app.set_rows(join_with_config(&cfg)?);
                    }
                }
                Action::Kill => {
                    if let Some(row) = app.selected() {
                        let _ =
                            proc::kill::kill_record(&row.record, Duration::from_millis(1), true);
                        app.set_rows(join_with_config(&cfg)?);
                    }
                }
                Action::Start => {
                    if let Some(row) = app.selected() {
                        if let Some(name) = &row.service_name {
                            if let Some(svc) = cfg.services.get(name) {
                                let _ = std::process::Command::new("zsh")
                                    .arg("-lc")
                                    .arg(svc.start.clone().unwrap_or_default())
                                    .current_dir(&svc.repo)
                                    .status();
                                app.set_rows(join_with_config(&cfg)?);
                            }
                        }
                    }
                }
                Action::OpenConfig => {
                    let _ = crate::open_config(config_path);
                }
                Action::Filter => {
                    app.show_filter = !app.show_filter;
                }
                Action::None => {}
            }
        }
    }

    disable_raw_mode()?;
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
