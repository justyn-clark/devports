use crate::scan::model::JoinedPortRecord;
use ratatui::widgets::TableState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Filter,
}

#[derive(Debug)]
pub struct App {
    pub rows: Vec<JoinedPortRecord>,
    pub filtered_indices: Vec<usize>,
    pub table_state: TableState,
    pub filter: String,
    pub mode: InputMode,
    pub show_help: bool,
    pub status: String,
}

impl App {
    pub fn new(rows: Vec<JoinedPortRecord>) -> Self {
        let mut app = Self {
            rows,
            filtered_indices: Vec::new(),
            table_state: TableState::default(),
            filter: String::new(),
            mode: InputMode::Normal,
            show_help: false,
            status: "Arrows navigate. Enter opens the selected service. ? shows all controls."
                .to_string(),
        };
        app.apply_filter();
        app
    }

    pub fn set_rows(&mut self, rows: Vec<JoinedPortRecord>) {
        self.rows = rows;
        self.apply_filter();
    }

    pub fn selected(&self) -> Option<&JoinedPortRecord> {
        self.selected_row_index().and_then(|index| self.rows.get(index))
    }

    pub fn move_down(&mut self) {
        self.move_by(1);
    }

    pub fn move_up(&mut self) {
        self.move_by(-1);
    }

    pub fn page_down(&mut self) {
        self.move_by(10);
    }

    pub fn page_up(&mut self) {
        self.move_by(-10);
    }

    pub fn move_home(&mut self) {
        self.select_position(Some(0));
    }

    pub fn move_end(&mut self) {
        self.select_position(self.filtered_indices.len().checked_sub(1));
    }

    pub fn start_filter(&mut self) {
        self.mode = InputMode::Filter;
        self.show_help = false;
        self.status = "Filter mode: type to narrow results, Enter or Esc to return.".to_string();
    }

    pub fn finish_filter(&mut self) {
        self.mode = InputMode::Normal;
        self.status = self.filter_status();
    }

    pub fn cancel_filter(&mut self) {
        self.mode = InputMode::Normal;
        self.status = self.filter_status();
    }

    pub fn push_filter_char(&mut self, ch: char) {
        self.filter.push(ch);
        self.apply_filter();
        self.status = self.filter_status();
    }

    pub fn pop_filter_char(&mut self) {
        self.filter.pop();
        self.apply_filter();
        self.status = self.filter_status();
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
        if self.show_help {
            self.mode = InputMode::Normal;
            self.status = "Help: review controls, then press ? or Esc to close.".to_string();
        } else {
            self.status = self.filter_status();
        }
    }

    pub fn dismiss_overlay(&mut self) {
        if self.show_help {
            self.show_help = false;
            self.status = self.filter_status();
            return;
        }

        if self.mode == InputMode::Filter {
            self.cancel_filter();
        }
    }

    pub fn set_status(&mut self, status: impl Into<String>) {
        self.status = status.into();
    }

    pub fn visible_rows(&self) -> Vec<&JoinedPortRecord> {
        self.filtered_indices
            .iter()
            .filter_map(|index| self.rows.get(*index))
            .collect()
    }

    pub fn result_count(&self) -> usize {
        self.filtered_indices.len()
    }

    pub fn total_count(&self) -> usize {
        self.rows.len()
    }

    fn apply_filter(&mut self) {
        let current_row = self.selected_row_index();
        let needle = self.filter.trim().to_lowercase();

        self.filtered_indices = self
            .rows
            .iter()
            .enumerate()
            .filter(|(_, row)| needle.is_empty() || row_matches_filter(row, &needle))
            .map(|(index, _)| index)
            .collect();

        let next_position = current_row
            .and_then(|index| self.filtered_indices.iter().position(|candidate| *candidate == index))
            .or_else(|| (!self.filtered_indices.is_empty()).then_some(0));

        self.select_position(next_position);
    }

    fn move_by(&mut self, delta: isize) {
        if self.filtered_indices.is_empty() {
            self.select_position(None);
            return;
        }

        let current = self.table_state.selected().unwrap_or(0) as isize;
        let max = self.filtered_indices.len().saturating_sub(1) as isize;
        let next = (current + delta).clamp(0, max) as usize;
        self.select_position(Some(next));
    }

    fn select_position(&mut self, position: Option<usize>) {
        self.table_state.select(position);
    }

    fn selected_row_index(&self) -> Option<usize> {
        self.table_state
            .selected()
            .and_then(|position| self.filtered_indices.get(position))
            .copied()
    }

    fn filter_status(&self) -> String {
        if self.filter.trim().is_empty() {
            format!(
                "{} listeners loaded. Enter opens service URLs, / starts filtering.",
                self.total_count()
            )
        } else {
            format!(
                "Filter '{}' matched {} of {} listeners.",
                self.filter,
                self.result_count(),
                self.total_count()
            )
        }
    }
}

fn row_matches_filter(row: &JoinedPortRecord, needle: &str) -> bool {
    row.record.command.to_lowercase().contains(needle)
        || row
            .service_name
            .as_deref()
            .unwrap_or("")
            .to_lowercase()
            .contains(needle)
        || row.record.port.to_string().contains(needle)
        || row.record.pid.to_string().contains(needle)
        || row
            .record
            .cwd
            .as_deref()
            .map(|path| path.display().to_string().to_lowercase().contains(needle))
            .unwrap_or(false)
        || row
            .record
            .repo_root
            .as_deref()
            .map(|path| path.display().to_string().to_lowercase().contains(needle))
            .unwrap_or(false)
}
