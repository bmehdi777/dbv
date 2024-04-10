use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub enum Keys {
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Enter,
    Backspace,
    Esc,

    AltChar(char),
    CtrlChar(char),
    Char(char),
    Unknown,
}

impl From<KeyEvent> for Keys {
    fn from(value: KeyEvent) -> Self {
        match value {
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => Keys::CtrlChar(c),
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::ALT,
                ..
            } => Keys::AltChar(c),
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
                code: KeyCode::Up, ..
            } => Keys::ArrowUp,
            KeyEvent {
                code: KeyCode::Down,
                ..
            } => Keys::ArrowDown,
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => Keys::Enter,
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } => Keys::Backspace,
            KeyEvent {
                code: KeyCode::Esc,
                ..
            } => Keys::Esc,

            _ => Keys::Unknown,
        }
    }
}
