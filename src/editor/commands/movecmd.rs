use std::convert::TryFrom;
use crossterm::event::{
    KeyCode::{
        Left,
        Right,
        Up,
        Down,
        Home,
        End,
        PageUp,
        PageDown,
    },
    KeyEvent,
    KeyModifiers,
};

#[derive(Clone, Copy)]
pub enum Move {
    PageUp,
    PageDown,
    StartOfLine,
    EndOfLine,
    Up,
    Left,
    Right,
    Down,
}

impl TryFrom<KeyEvent> for Move {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code,
            modifiers,
            ..
        } = event;

        if modifiers == KeyModifiers::NONE {
            match code {
                Up => {
                    return Ok(Self::Up);
                },
                Down => {
                    return Ok(Self::Down);
                },
                Left => {
                    return Ok(Self::Left);
                },
                Right => {
                    return Ok(Self::Right);
                },
                PageDown => {
                    return Ok(Self::PageDown);
                },
                PageUp => {
                    return Ok(Self::PageUp);
                },
                Home => {
                    return Ok(Self::StartOfLine);
                },
                End => {
                    return Ok(Self::EndOfLine);
                },
                _ => {
                    return Err(String::new());
                },
            }
        } else {
            return Err(String::new());
        }
    }
}
