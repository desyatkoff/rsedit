use std::io::{
    stdout,
    Write,
    Error,
};
use core::fmt::Display;
use crossterm::{
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        size,
        Clear,
        ClearType,
    },
    cursor::{
        MoveTo,
        Hide,
        Show,
    },
    style::Print,
    queue,
    Command,
};

pub struct Terminal;

#[derive(Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub column: usize,
    pub row: usize,
}

impl Terminal {
    pub fn init() -> Result<(), Error> {
        enable_raw_mode()?;

        Self::clear_all()?;
        Self::move_cursor_to(
            Position {
                column: 0,
                row: 0,
            }
        )?;
        Self::execute()?;

        return Ok(());
    }

    pub fn kill() -> Result<(), Error> {
        Self::execute()?;

        disable_raw_mode()?;

        return Ok(());
    }

    pub fn clear_all() -> Result<(), Error> {
        Self::queue_cmd(
            Clear(ClearType::All),
        )?;

        return Ok(());
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_cmd(
            Clear(ClearType::CurrentLine),
        )?;

        return Ok(());
    }

    pub fn move_cursor_to(pos: Position) -> Result<(), Error> {
        Self::queue_cmd(
            MoveTo(
                pos.column as u16,
                pos.row as u16
            )
        )?;

        return Ok(());
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_cmd(Hide)?;

        return Ok(());
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_cmd(Show)?;

        return Ok(());
    }

    pub fn print<T: Display>(s: T) -> Result<(), Error> {
        Self::queue_cmd(Print(s))?;

        return Ok(());
    }

    pub fn size() -> Result<Size, Error> {
        let (
            width_u16,
            height_u16
        ) = size()?;

        return Ok(
            Size {
                width: width_u16 as usize,
                height: height_u16 as usize
            }
        );
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;

        return Ok(());
    }

    pub fn queue_cmd<T: Command>(cmd: T) -> Result<(), Error> {
        queue!(
            stdout(),
            cmd
        )?;

        return Ok(());
    }
}

