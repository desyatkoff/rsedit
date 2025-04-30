use std::convert::TryFrom;
use crossterm::event::{
    KeyCode::Char,
    KeyCode,
    KeyEvent,
    KeyModifiers,
};
use super::Size;

#[derive(Clone, Copy)]
pub enum System {
    Save,
    Resize(Size),
    Quit,
    Dismiss,
    Search,
}

impl TryFrom<KeyEvent> for System {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code,
            modifiers,
            ..
        } = event;

        if modifiers == KeyModifiers::CONTROL {
            match code {
                Char('q') => {
                    return Ok(
                        Self::Quit
                    );
                },
                Char('s') => {
                    return Ok(
                        Self::Save
                    );
                },
                Char('f') => {
                    return Ok(
                        Self::Search
                    );
                },
                _ => {
                    return Err(String::new());
                },
            }
        } else if matches!(code, KeyCode::Esc) && modifiers == KeyModifiers::NONE {
            return Ok(
                Self::Dismiss
            );
        } else {
            return Err(String::new());
        }
    }
}
