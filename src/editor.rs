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
use commands::EditorCmd;
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

        let args: Vec<String> = env::args().collect();

        if let Some(file) = args.get(1) {
            editor.view.load(file);
        }

        editor.update_status();
        editor.update_hint(
            String::from(
                "[ CONTROL + S -> SAVE ] [ CONTROL + Q -> QUIT ]"
            )
        );

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
                    } else if let EditorCmd::Resize(size) = cmd {
                        self.resize(size);
                    } else {
                        self.view.handle_command(cmd);
                    }
                },
                Err(_error) => {},
            }
        }
    }

    pub fn update_status(&mut self) {
        let status = self.view.get_current_status();
        let title = format!(
            "Rsedit :: {}",
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

    pub fn update_hint(&mut self, hint: String) {
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

