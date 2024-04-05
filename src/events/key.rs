use crossterm::event::{KeyCode, KeyEvent};

pub enum Keys {
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Char(char),
    Unknown,
}

impl From<KeyEvent> for Keys {
    fn from(value: KeyEvent) -> Self {
        match value {
            KeyEvent {
                code: KeyCode::Char(c),
                ..
            } => Keys::Char(c),
            KeyEvent {
                code: KeyCode::Left,
                ..
            } => Keys::ArrowLeft,
            KeyEvent {
                code: KeyCode::Right,
                ..
            } => Keys::ArrowRight,
            KeyEvent {
                code: KeyCode::Up,
                ..
            } => Keys::ArrowUp,
            KeyEvent {
                code: KeyCode::Down,
                ..
            } => Keys::ArrowDown,
            _ => Keys::Unknown,
        }
    }
}
