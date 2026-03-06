use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{
    Block, Borders, Cell, Clear, Paragraph, Row, Table, Wrap,
};

use super::app::App;

pub fn draw(frame: &mut Frame<'_>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
        .split(chunks[1]);

    let summary = Paragraph::new(Line::from(vec![
        "devports tui".into(),
        "  ".into(),
        format!("visible {}/{}", app.result_count(), app.total_count()).into(),
        "  ".into(),
        selected_line(app).into(),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Session"));
    frame.render_widget(summary, chunks[0]);

    let rows = app.visible_rows().into_iter().map(|row| {
        Row::new(vec![
            Cell::from(row.record.port.to_string()),
            Cell::from(row.service_name.clone().unwrap_or_else(|| "-".to_string())),
            Cell::from(match_label(row)),
            Cell::from(row.record.pid.to_string()),
            Cell::from(row.record.command.clone()),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(6),
            Constraint::Length(24),
            Constraint::Length(10),
            Constraint::Length(8),
            Constraint::Min(20),
        ],
    )
    .header(
        Row::new(vec!["PORT", "SERVICE", "STATE", "PID", "COMMAND"]).style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    )
    .row_highlight_style(
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol(">> ")
    .block(Block::default().title("Listeners").borders(Borders::ALL));

    frame.render_stateful_widget(table, body[0], &mut app.table_state);

    let details = Paragraph::new(details_text(app))
        .block(Block::default().title("Details").borders(Borders::ALL))
        .wrap(Wrap { trim: false });
    frame.render_widget(details, body[1]);

    let controls = Paragraph::new(
        "Enter open | k term | K kill | s start | r rescan | e config | o open | / filter | ? help | q quit",
    )
    .block(Block::default().borders(Borders::ALL).title("Controls"))
    .wrap(Wrap { trim: true });
    frame.render_widget(controls, chunks[2]);

    let filter_title = if app.mode == super::app::InputMode::Filter {
        "Filter (editing)"
    } else {
        "Filter / Status"
    };
    let filter_text = if app.mode == super::app::InputMode::Filter {
        format!("filter: {}_", app.filter)
    } else if app.filter.is_empty() {
        app.status.clone()
    } else {
        format!("filter: {} | {}", app.filter, app.status)
    };
    let footer = Paragraph::new(filter_text)
        .block(Block::default().borders(Borders::ALL).title(filter_title))
        .wrap(Wrap { trim: false });
    frame.render_widget(footer, chunks[3]);

    if app.show_help {
        let area = centered_rect(74, 60, frame.area());
        frame.render_widget(Clear, area);
        let help = Paragraph::new(help_overlay())
            .block(Block::default().title("Keyboard Help").borders(Borders::ALL))
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Left);
        frame.render_widget(help, area);
    }
}

fn match_label(row: &crate::scan::model::JoinedPortRecord) -> &'static str {
    match (row.service_name.as_ref(), row.configured_port) {
        (Some(_), Some(expected)) if expected == row.record.port => "ok",
        (Some(_), Some(_)) => "mismatch",
        _ => "unmapped",
    }
}

fn selected_line(app: &App) -> String {
    app.selected()
        .map(|row| format!("selected {}:{}", row.service_name.as_deref().unwrap_or("-"), row.record.port))
        .unwrap_or_else(|| "selected none".to_string())
}

fn details_text(app: &App) -> Text<'static> {
    let Some(row) = app.selected() else {
        return Text::from("No listeners match the current view.\n\nPress / to filter or r to rescan.");
    };

    let mut lines = Vec::new();
    lines.push(Line::from(format!(
        "Service: {}",
        row.service_name.as_deref().unwrap_or("unmapped")
    )));
    lines.push(Line::from(format!("Port: {}", row.record.port)));
    lines.push(Line::from(format!("State: {}", match_label(row))));
    lines.push(Line::from(format!("PID: {}", row.record.pid)));
    lines.push(Line::from(format!(
        "PGID: {}",
        row.record
            .pgid
            .map(|value| value.to_string())
            .unwrap_or_else(|| "-".to_string())
    )));
    lines.push(Line::from(format!("Protocol: {}", row.record.protocol)));
    lines.push(Line::from(format!("Command: {}", row.record.command)));
    lines.push(Line::from(format!(
        "URL: {}",
        crate::service_url(row.record.port)
    )));
    lines.push(Line::from(format!(
        "Repo root: {}",
        row.record
            .repo_root
            .as_deref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "-".to_string())
    )));
    lines.push(Line::from(format!(
        "Working dir: {}",
        row.record
            .cwd
            .as_deref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "-".to_string())
    )));
    Text::from(lines)
}

fn help_overlay() -> Text<'static> {
    Text::from(vec![
        Line::from("Navigation"),
        Line::from("  Up/Down or j       Move one row"),
        Line::from("  Home/End or g/G    Jump to first or last row"),
        Line::from("  PageUp/PageDown    Move by larger steps"),
        Line::from(""),
        Line::from("Actions"),
        Line::from("  Enter or o         Open selected service URL"),
        Line::from("  k                  Graceful terminate selected listener"),
        Line::from("  K                  Force kill selected listener"),
        Line::from("  s                  Start the selected configured service"),
        Line::from("  r                  Rescan current listeners"),
        Line::from("  e                  Open the config file"),
        Line::from(""),
        Line::from("Filtering"),
        Line::from("  /                  Enter filter mode"),
        Line::from("  type               Update the filter while editing"),
        Line::from("  Backspace          Remove the last filter character"),
        Line::from("  Enter or Esc       Exit filter mode"),
        Line::from(""),
        Line::from("Other"),
        Line::from("  ? or F1            Toggle this help"),
        Line::from("  q                  Quit"),
    ])
}

fn centered_rect(horizontal: u16, vertical: u16, area: Rect) -> Rect {
    let [area] = Layout::horizontal([Constraint::Percentage(horizontal)])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([Constraint::Percentage(vertical)])
        .flex(Flex::Center)
        .areas(area);
    area
}
