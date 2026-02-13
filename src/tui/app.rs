use crate::config::Config;
use crate::scan::model::JoinedPortRecord;

#[derive(Debug)]
pub struct App {
    pub rows: Vec<JoinedPortRecord>,
    pub selected: usize,
    pub filter: String,
    pub show_filter: bool,
}

impl App {
    pub fn new(rows: Vec<JoinedPortRecord>) -> Self {
        Self {
            rows,
            selected: 0,
            filter: String::new(),
            show_filter: false,
        }
    }

    pub fn set_rows(&mut self, rows: Vec<JoinedPortRecord>) {
        self.rows = rows;
        if self.selected >= self.rows.len() {
            self.selected = self.rows.len().saturating_sub(1);
        }
    }

    pub fn selected(&self) -> Option<&JoinedPortRecord> {
        self.rows.get(self.selected)
    }

    pub fn move_down(&mut self) {
        if self.selected + 1 < self.rows.len() {
            self.selected += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn filtered_rows(&self, cfg: &Config) -> Vec<JoinedPortRecord> {
        if self.filter.trim().is_empty() {
            return self.rows.clone();
        }

        let needle = self.filter.to_lowercase();
        self.rows
            .iter()
            .filter(|row| {
                row.record.command.to_lowercase().contains(&needle)
                    || row
                        .service_name
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&needle)
                    || cfg
                        .services
                        .keys()
                        .any(|k| k == row.service_name.as_deref().unwrap_or(""))
            })
            .cloned()
            .collect()
    }
}
