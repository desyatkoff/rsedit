mod terminal;
mod view;
mod commands;

use std::{
    io::Error,
    env,
    panic::{
        set_hook,
        take_hook,
    },
};
use crossterm::event::{
    read,
    Event,
    KeyEvent,
    KeyEventKind,
};
use terminal::Terminal;
use view::View;
use commands::EditorCmd;

pub struct Editor {
    should_quit: bool,
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

    fn eval_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(
                KeyEvent {
                    kind,
                    ..
                }
            ) => {
                kind == &KeyEventKind::Press
            },
            Event::Resize(
                _,
                _
            ) => {
                true
            },
            _ => {
                false
            },
        };

        if should_process {
            match EditorCmd::try_from(event) {
                Ok(cmd) => {
                    if matches!(cmd, EditorCmd::Quit) {
                        self.should_quit = true;
                    } else {
                        self.view.handle_command(cmd);
                    }
                },
                Err(_error) => {},
            }
        }
    }

    fn refresh_screen(&mut self) {
        Terminal::hide_cursor();

        self.view.render();

        Terminal::move_cursor_to(self.view.get_cursor_position());
        Terminal::show_cursor();
        Terminal::execute();
    }
}

