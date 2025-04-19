mod terminal;
mod view;

use std::io::Error;
use core::cmp::min;
use crossterm::{
    event::{
        read,
        Event,
        Event::Key,
        KeyCode,
        KeyCode::Char,
        KeyEvent,
        KeyEventKind,
        KeyModifiers,
    }
};
use terminal::{
    Terminal,
    Size,
    Position,
};
use view::View;

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}

impl Editor {
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

            self.eval_event(&event)?;
        }

        return Ok(());
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location {
            mut x,
            mut y
        } = self.location;
        let Size {
            width,
            height
        } = Terminal::size()?;

        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            },
            KeyCode::Down => {
                y = min(
                    height.saturating_sub(1),
                    y.saturating_add(1),
                );
            },
            KeyCode::Left => {
                x = x.saturating_sub(1);
            },
            KeyCode::Right => {
                x = min(
                    width.saturating_sub(1),
                    x.saturating_add(1)
                );
            },
            KeyCode::PageUp => {
                y = 0;
            },
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            },
            KeyCode::Home => {
                x = 0;
            },
            KeyCode::End => {
                x = width.saturating_sub(1);
            },
            _ => {
                return Ok(());
            },
        }

        self.location = Location {
            x,
            y
        };

        return Ok(());
    }

    fn eval_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(
            KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }
        ) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right | KeyCode::PageUp | KeyCode::PageDown | KeyCode::Home | KeyCode::End => {
                    self.move_point(*code)?;
                },
                _ => {
                    return Ok(());
                }
            }
        }

        return Ok(());
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(Position::default())?;

        if self.should_quit {
            Terminal::clear_all()?;
        } else {
            View::render()?;

            Terminal::move_cursor_to(
                Position {
                    column: self.location.x,
                    row: self.location.y
                }
            )?;
        }

        Terminal::show_cursor()?;
        Terminal::execute()?;

        return Ok(());
    }
}

