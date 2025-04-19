use std::io::Error;
use super::terminal::{
    Terminal,
    Size
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View;

impl View {
    pub fn render() -> Result<(), Error> {
        let Size {
            width: _,
            height
        } = Terminal::size()?;

        Terminal::clear_line()?;

        for current_line in 0..height {
            Terminal::clear_line()?;

            if current_line == height / 3 {
                Self::draw_welcome_msg()?;
            } else {
                Self::draw_empty_line()?;
            }

            if current_line.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }

        return Ok(());
    }

    fn draw_welcome_msg() -> Result<(), Error> {
        let mut welcome_msg = format!("Welcome to the Rsedit v{VERSION}!");
        let width = Terminal::size()?.width;
        let length = welcome_msg.len();
        let padding = (width.saturating_sub(length)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_msg = format!("~{spaces}{welcome_msg}");
        welcome_msg.truncate(width);

        Terminal::print(&welcome_msg)?;

        return Ok(());
    }

    fn draw_empty_line() -> Result<(), Error> {
        Terminal::print("~")?;

        return Ok(());
    }
}

