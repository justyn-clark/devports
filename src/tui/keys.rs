use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::app::InputMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    Rescan,
    SoftKill,
    HardKill,
    Start,
    OpenConfig,
    OpenService,
    StartFilter,
    ToggleHelp,
    Down,
    Up,
    PageDown,
    PageUp,
    Home,
    End,
    Confirm,
    Cancel,
    Backspace,
    Input(char),
    None,
}

pub fn map_key(key: KeyEvent, mode: InputMode, show_help: bool) -> Action {
    if show_help {
        return match key.code {
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') | KeyCode::Char('?') | KeyCode::F(1) => {
                Action::Cancel
            }
            _ => Action::None,
        };
    }

    if mode == InputMode::Filter {
        return match key.code {
            KeyCode::Esc | KeyCode::Enter => Action::Confirm,
            KeyCode::Backspace => Action::Backspace,
            KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => Action::Input(c),
            _ => Action::None,
        };
    }

    match key.code {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Char('r') => Action::Rescan,
        KeyCode::Char('k') => Action::SoftKill,
        KeyCode::Char('K') => Action::HardKill,
        KeyCode::Char('s') => Action::Start,
        KeyCode::Char('e') => Action::OpenConfig,
        KeyCode::Char('o') | KeyCode::Enter => Action::OpenService,
        KeyCode::Char('/') => Action::StartFilter,
        KeyCode::Char('?') | KeyCode::F(1) => Action::ToggleHelp,
        KeyCode::Down | KeyCode::Char('j') => Action::Down,
        KeyCode::Up => Action::Up,
        KeyCode::PageDown => Action::PageDown,
        KeyCode::PageUp => Action::PageUp,
        KeyCode::Home | KeyCode::Char('g') => Action::Home,
        KeyCode::End | KeyCode::Char('G') => Action::End,
        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::PageDown,
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::PageUp,
        KeyCode::Esc => Action::Cancel,
        _ => Action::None,
    }
}
