use std::io::stdout;
use crossterm::{
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        size,
        Clear,
        ClearType,
    },
    cursor::MoveTo,
    execute,
};

pub struct Terminal {}

impl Terminal {
    pub fn init() -> Result<(), std::io::Error> {
        enable_raw_mode()?;

        Self::clear()?;
        Self::move_cursor_to(0, 0)?;

        return Ok(());
    }

    pub fn kill() -> Result<(), std::io::Error> {
        disable_raw_mode()?;

        return Ok(());
    }

    pub fn clear() -> Result<(), std::io::Error> {
        execute!(
            stdout(),
            Clear(ClearType::All),
        )?;

        return Ok(());
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), std::io::Error> {
        execute!(
            stdout(),
            MoveTo(x, y)
        )?;

        return Ok(());
    }

    pub fn size() -> Result<(u16, u16), std::io::Error> {
        return size();
    }
}

