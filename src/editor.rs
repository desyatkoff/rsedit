mod terminal;
mod view;
mod commands;
mod statusbar;
mod filestatus;
mod fileinfo;
mod hintbar;
mod uielements;

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
use terminal::{
    Terminal,
    Size,
};
use view::View;
use commands::{
    Command,
    Command::{
        Edit,
        Move,
        System,
    },
    System::{
        Quit,
        Resize,
        Save,
    },
};
use statusbar::StatusBar;
use filestatus::FileStatus;
use hintbar::HintBar;
use uielements::{
    UIElement,
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    statusbar: StatusBar,
    hintbar: HintBar,
    title: String,
    terminal_size: Size,
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

        let mut editor = Self::default();
        editor.resize(
            Terminal::size().unwrap_or_default()
        );
        editor.update_hint("[ CONTROL + S -> SAVE ] [ CONTROL + Q -> QUIT ]");

        let args: Vec<String> = env::args().collect();

        if let Some(file) = args.get(1) {
            if editor.view.load(file).is_err() {
                editor.update_hint("[ ERROR OPENING FILE. YOUR CHANGES WILL NOT BE SAVED ]")
            }
        }

        editor.update_status();

        return Ok(editor);
    }

    pub fn run(&mut self) {
        loop {
            self.update_screen();

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

            let status = self.view.get_current_status();

            self.statusbar.update_status(status);
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
            Event::Resize(_, _) => {
                true
            },
            _ => {
                false
            },
        };

        if should_process {
            if let Ok(command) = Command::try_from(event) {
                self.process_command(command);
            }
        }
    }

    fn process_command(&mut self, command: Command) {
        match command {
            System(Quit) => {
                return self.handle_quit();
            },
            System(Resize(size)) => {
                return self.resize(size);
            },
            System(Save) => {
                return self.handle_save();
            },
            Edit(edit_cmd) => {
                return self.view.handle_edit_command(edit_cmd);
            },
            Move(move_cmd) => {
                return self.view.handle_move_command(move_cmd);
            }
        }
    }

    fn handle_quit(&mut self) {
        if !self.view.get_current_status().modified ||
            self.view.get_current_status().file_name.as_deref().unwrap() == "UNTITLED" {
            self.hintbar.update_hint("[ QUITTING ]");
            self.should_quit = true;
        } else if self.view.get_current_status().modified {
            self.hintbar.update_hint("[ THIS FILE HAS UNSAVED CHANGES ]");
        }
    }

    fn handle_save(&mut self) {
        if self.view.save().is_ok() {
            self.hintbar.update_hint("[ SUCCESSFULLY SAVED THIS FILE ]");
        } else {
            self.hintbar.update_hint("[ ERROR SAVING THIS FILE ]");
        }
    }

    pub fn update_status(&mut self) {
        let status = self.view.get_current_status();
        let title = format!(
            "[ RSEDIT ] :: [ {} ]",
            status.file_name
                .as_deref()
                .unwrap(),
        );

        self.statusbar.update_status(status);

        if title != self.title &&
            matches!(
                Terminal::set_title(&title),
                Ok(())
            ) {
            self.title = title;
        }
    }

    pub fn update_hint(&mut self, hint: &str) {
        self.hintbar.update_hint(hint);
    }

    fn update_screen(&mut self) {
        Terminal::hide_cursor();

        if self.terminal_size.height > 0 {
            self.statusbar.render(
                self.terminal_size.height.saturating_sub(2)
            );
        }

        if self.terminal_size.height > 1 {
            self.hintbar.render(
                self.terminal_size.height.saturating_sub(1)
            );
        }

        if self.terminal_size.height > 2 {
            self.view.render(0);
        }

        Terminal::move_cursor_to(
            self.view.get_cursor_position()
        );
        Terminal::show_cursor();
        Terminal::execute();
    }

    fn resize(&mut self, new_size: Size) {
        self.terminal_size = new_size;
        self.view.resize(
            Size {
                width: new_size.width,
                height: new_size.height.saturating_sub(2),
            }
        );
        self.statusbar.resize(
            Size {
                width: new_size.width,
                height: 1,
            }
        );
        self.hintbar.resize(
            Size {
                width: new_size.width,
                height: 1,
            }
        );
    }
}

