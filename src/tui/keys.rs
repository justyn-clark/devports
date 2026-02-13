use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    Rescan,
    Term,
    Kill,
    Start,
    OpenConfig,
    Filter,
    Down,
    Up,
    None,
}

pub fn map_key(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Char('r') => Action::Rescan,
        KeyCode::Char('k') => Action::Term,
        KeyCode::Char('K') => Action::Kill,
        KeyCode::Char('s') => Action::Start,
        KeyCode::Char('e') => Action::OpenConfig,
        KeyCode::Char('/') => Action::Filter,
        KeyCode::Down => Action::Down,
        KeyCode::Up => Action::Up,
        _ => Action::None,
    }
}
