mod terminal;

use std::io::Error;
use crossterm::{
    event::{
        read,
        Event,
        Event::Key,
        KeyCode::Char,
        KeyEvent,
        KeyModifiers
    }
};
use terminal::{
    Terminal,
    Size,
    Position,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn new() -> Self {
        return Self { should_quit: false };
    }

    pub fn run(&mut self) {
        Terminal::init().unwrap();

        let result = self.repl();

        Terminal::kill().unwrap();

        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }

            let event = read()?;

            self.eval_event(&event);
        }

        return Ok(());
    }

    fn eval_event(&mut self, event: &Event) {
        if let Key(
            KeyEvent {
                code,
                modifiers,
                ..
            }
        ) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                _ => {
                    return;
                }
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;

        if self.should_quit {
            Terminal::clear_all()?;
        } else {
            Self::draw_lines()?;

            Terminal::move_cursor_to(
                Position {
                    x: 0,
                    y: 0
                }
            )?;
        }

        Terminal::show_cursor()?;
        Terminal::execute()?;

        return Ok(());
    }

    fn draw_welcome_msg() -> Result<(), Error> {
        let mut welcome_msg = format!("Welcome to the Rsedit v{VERSION}!");
        let width = Terminal::size()?.width as usize;
        let length = welcome_msg.len();
        let padding = (width - length) / 2;
        let spaces = " ".repeat(padding - 1);

        welcome_msg = format!("~{spaces}{welcome_msg}");
        welcome_msg.truncate(width);

        Terminal::print(&welcome_msg)?;

        return Ok(());
    }

    fn draw_empty_line() -> Result<(), Error> {
        Terminal::print("~")?;

        return Ok(());
    }

    fn draw_lines() -> Result<(), Error> {
        let Size {
            height,
            ..
        } = Terminal::size()?;

        for current_line in 0..height {
            Terminal::clear_line()?;

            if current_line == height / 3 {
                Self::draw_welcome_msg()?;
            } else {
                Self::draw_empty_line()?;
            }

            if current_line + 1 < height {
                Terminal::print("\r\n")?;
            }
        }

        return Ok(());
    }
}

