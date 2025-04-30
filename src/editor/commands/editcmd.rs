use std::convert::TryFrom;
use crossterm::event::{
    KeyCode::{
        Char,
        Tab,
        Enter,
        Backspace,
        Delete,
    },
    KeyEvent,
    KeyModifiers,
};

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
