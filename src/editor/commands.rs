use std::convert::TryFrom;
use crossterm::event::{
    KeyCode::{
        Char,
        Backspace,
        Delete,
        Left,
        Right,
        Up,
        Down,
        Home,
        End,
        Tab,
        Enter,
        PageUp,
        PageDown,
    },
    KeyEvent,
    KeyModifiers,
    Event,
};
use super::terminal::Size;

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

#[derive(Clone, Copy)]
pub enum Edit {
    InsertCharacter(char),
    InsertTab,
    InsertLine,
    DeletePrevious,
    DeleteNext,
}

impl TryFrom<KeyEvent> for Edit {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        match (event.code, event.modifiers) {
            (
                Char(character),
                KeyModifiers::NONE | KeyModifiers::SHIFT
            ) => {
                return Ok(
                    Self::InsertCharacter(character)
                );
            },
            (
                Tab,
                KeyModifiers::NONE
            ) => {
                return Ok(
                    Self::InsertTab
                );
            },
            (
                Enter,
                KeyModifiers::NONE
            ) => {
                return Ok(
                    Self::InsertLine
                );
            },
            (
                Backspace,
                KeyModifiers::NONE
            ) => {
                return Ok(
                    Self::DeletePrevious
                );
            },
            (
                Delete,
                KeyModifiers::NONE
            ) => {
                return Ok(
                    Self::DeleteNext
                );
            },
            _ => {
                return Err(String::new());
            },
        }
    }
}

#[derive(Clone, Copy)]
pub enum System {
    Save,
    Resize(Size),
    Quit,
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
                _ => {
                    return Err(String::new());
                },
            }
        } else {
            return Err(String::new());
        }
    }
}

#[derive(Clone, Copy)]
pub enum Command {
    Move(Move),
    Edit(Edit),
    System(System),
}

impl TryFrom<Event> for Command {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(key_event) => {
                return Edit::try_from(key_event)
                    .map(
                        Command::Edit
                    )
                    .or_else(
                        |_| Move::try_from(key_event)
                            .map(Command::Move)
                    )
                    .or_else(
                        |_| System::try_from(key_event)
                        .map(Command::System)
                    );
            },
            Event::Resize(width_u16, height_u16) => {
                return Ok(
                    Self::System(
                        System::Resize(
                            Size {
                                width: width_u16 as usize,
                                height: height_u16 as usize,
                            }
                        )
                    )
                );
            },
            _ => {
                return Err(String::new());
            }
        }
    }
}
