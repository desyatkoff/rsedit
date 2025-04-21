mod terminal;
mod view;
mod commands;
mod statusbar;
mod filestatus;
mod fileinfo;

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
use statusbar::StatusBar;
use filestatus::FileStatus;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    view: View,
    statusbar: StatusBar,
    title: String,
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

        let mut editor = Self {
            should_quit: false,
            view: View::new(2),
            statusbar: StatusBar::new(1),
            title: String::new(),
        };

        let args: Vec<String> = env::args().collect();

        if let Some(file) = args.get(1) {
            editor.view.load(file);
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

                        if let EditorCmd::Resize(size) = cmd {
                            self.statusbar.resize(size);
                        }
                    }
                },
                Err(_error) => {},
            }
        }
    }

    pub fn update_status(&mut self) {
        let status = self.view.get_current_status();
        let title = format!(
            "Rsedit :: {:?}",
            status.file_name,
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

    fn update_screen(&mut self) {
        Terminal::hide_cursor();

        self.view.render();
        self.statusbar.render();

        Terminal::move_cursor_to(self.view.get_cursor_position());
        Terminal::show_cursor();
        Terminal::execute();
    }
}

