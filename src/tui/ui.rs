use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};

use super::app::App;

pub fn draw(frame: &mut Frame<'_>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(2),
            Constraint::Length(2),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let rows = app.rows.iter().enumerate().map(|(i, row)| {
        let style = if i == app.selected {
            Style::default().fg(Color::Black).bg(Color::Cyan)
        } else {
            Style::default()
        };

        Row::new(vec![
            Cell::from(row.record.port.to_string()),
            Cell::from(row.service_name.clone().unwrap_or_else(|| "-".to_string())),
            Cell::from(row.record.pid.to_string()),
            Cell::from(
                row.record
                    .pgid
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "-".to_string()),
            ),
            Cell::from(row.record.command.clone()),
        ])
        .style(style)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(6),
            Constraint::Length(24),
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Min(20),
        ],
    )
    .header(Row::new(vec!["PORT", "SERVICE", "PID", "PGID", "COMMAND"]))
    .block(Block::default().title("ports tui").borders(Borders::ALL));

    frame.render_widget(table, chunks[0]);

    let help = Paragraph::new("k term | K kill | s start | r rescan | e open config | / filter | q quit")
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(help, chunks[1]);

    let filter = Paragraph::new(format!("filter: {}", app.filter));
    frame.render_widget(filter, chunks[2]);
}
