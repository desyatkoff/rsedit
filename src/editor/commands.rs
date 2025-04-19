use std::convert::TryFrom;
use crossterm::event::{
    Event,
    KeyCode,
    KeyEvent,
    KeyModifiers
};
use super::terminal::Size;

pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Left,
    Right,
    Down,
}
pub enum EditorCmd {
    Move(Direction),
    Resize(Size),
    Quit,
}

impl TryFrom<Event> for EditorCmd {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {code, modifiers, ..} ) => match (code, modifiers) {
                (
                    KeyCode::Char('q'),
                    KeyModifiers::CONTROL
                ) => {
                    return Ok(Self::Quit);
                },
                (
                    KeyCode::Up,
                    _
                ) => {
                    return Ok(Self::Move(Direction::Up));
                },
                (
                    KeyCode::Down,
                    _
                ) => {
                    return Ok(Self::Move(Direction::Down));
                },
                (
                    KeyCode::Left,
                    _
                ) => {
                    return Ok(Self::Move(Direction::Left));
                },
                (
                    KeyCode::Right,
                    _
                ) => {
                    return Ok(Self::Move(Direction::Right));
                },
                (
                    KeyCode::PageDown,
                    _
                ) => {
                    return Ok(Self::Move(Direction::PageDown));
                },
                (
                    KeyCode::PageUp,
                    _
                ) => {
                    return Ok(Self::Move(Direction::PageUp));
                },
                (
                    KeyCode::Home,
                    _
                ) => {
                    return Ok(Self::Move(Direction::Home));
                },
                (
                    KeyCode::End,
                    _
                ) => {
                    return Ok(Self::Move(Direction::End));
                },
                _ => {
                    return Err(String::from(""));
                },
            },
            Event::Resize(width_u16, height_u16) => {
                return Ok(
                    Self::Resize(
                        Size {
                            width: width_u16 as usize,
                            height: height_u16 as usize
                        }
                    )
                );
            },
            _ => {
                return Err(String::from(""));
            },
        }
    }
}

