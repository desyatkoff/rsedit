mod terminal;
mod view;

use std::{
    io::Error,
    env,
    panic::{
        set_hook,
        take_hook,
    },
};
use core::cmp::min;
use crossterm::event::{
    read,
    Event,
    KeyCode,
    KeyEvent,
    KeyEventKind,
    KeyModifiers,
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

pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();

        set_hook(
            Box::new(
                move |panic_info| {
                    let _ = Terminal::kill();

                    current_hook(panic_info);
                }
            )
        );

        Terminal::init()?;

        let mut view = View::default();
        let args: Vec<String> = env::args().collect();

        if let Some(file) = args.get(1) {
            view.load(file);
        }

        return Ok(
            Self {
                should_quit: false,
                location: Location::default(),
                view,
            }
        );
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();

            if self.should_quit {
                break;
            }

            match read() {
                Ok(event) => {
                    self.eval_event(event);
                },
                Err(error) => {
                    panic!("{error:?}");
                }
            }
        }
    }

    fn move_point(&mut self, key_code: KeyCode) {
        let Location { mut x, mut y } = self.location;
        let Size {
            width,
            height,
        } = Terminal::size().unwrap_or_default();

        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            },
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            },
            KeyCode::Left => {
                x = x.saturating_sub(1);
            },
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
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
                return;
            },
        }

        self.location = Location {
            x,
            y,
        };
    }

    fn eval_event(&mut self, event: Event) {
        match event {
            Event::Key(
                KeyEvent {
                    code,
                    kind: KeyEventKind::Press,
                    modifiers,
                    ..
                }
            ) => match (code, modifiers) {
                (
                    KeyCode::Char('q'),
                    KeyModifiers::CONTROL,
                ) => {
                    self.should_quit = true;
                },
                (
                    KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::PageUp
                    | KeyCode::PageDown
                    | KeyCode::Home
                    | KeyCode::End,
                    _,
                ) => {
                    self.move_point(code);
                },
                _ => {},
            },
            Event::Resize(width_u16, height_u16) => {
                self.view.resize(
                    Size {
                        width: width_u16 as usize,
                        height: height_u16 as usize,
                    }
                );
            },
            _ => {},
        }
    }

    fn refresh_screen(&mut self) {
        Terminal::hide_cursor();

        self.view.render();

        Terminal::move_cursor_to(
            Position {
                column: self.location.x,
                row: self.location.y,
            }
        );

        Terminal::show_cursor();

        Terminal::execute();
    }
}

