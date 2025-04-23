mod terminal;
mod view;
mod commands;
mod statusbar;
mod filestatus;
mod hintbar;
mod uielements;
mod commandbar;
mod line;
mod position;
mod size;

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
        Dismiss,
    },
    Edit::InsertLine,
};
use statusbar::StatusBar;
use filestatus::FileStatus;
use hintbar::HintBar;
use commandbar::CommandBar;
use uielements::{
    UIElement,
};
use line::Line;
use position::Position;
use size::Size;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    statusbar: StatusBar,
    hintbar: HintBar,
    commandbar: Option<CommandBar>,
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
        editor.update_hint("[ Control + S -> Save ] [ Control + Q -> Quit ]");

        let args: Vec<String> = env::args().collect();

        if let Some(file) = args.get(1) {
            if editor.view.load(file).is_err() {
                editor.update_hint("[ Error opening the file ]")
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
                if self.commandbar.is_none() {
                    self.handle_quit();
                }
            },
            System(Resize(size)) => {
                self.resize(size);
            },
            System(Save) => {
                if self.commandbar.is_none() {
                    self.handle_save();
                }
            },
            System(Dismiss) => {
                if self.commandbar.is_some() {
                    self.hide_command_prompt();
                    self.hintbar.update_hint("[ Action cancelled ]");
                }
            },
            Edit(edit_cmd) => {
                if let Some(cmdbar) = &mut self.commandbar {
                    if matches!(edit_cmd, InsertLine) {
                        let file_name = cmdbar.get_value();

                        self.hide_command_prompt();

                        self.save_file(
                            Some(&file_name)
                        );
                    } else {
                        cmdbar.handle_edit_command(edit_cmd);
                    }
                } else {
                    self.view.handle_edit_command(edit_cmd);
                }
            },
            Move(move_cmd) => {
                if self.commandbar.is_none() {
                    self.view.handle_move_command(move_cmd);
                }
            }
        }
    }

    fn show_command_prompt(&mut self) {
        let mut cmdbar = CommandBar::default();

        cmdbar.set_prompt("[ COMMAND ] :: Save the file as ");
        cmdbar.resize(
            Size {
                width: self.terminal_size.width,
                height: 1,
            }
        );
        cmdbar.set_needs_redraw(true);

        self.commandbar = Some(cmdbar);
    }

    fn hide_command_prompt(&mut self) {
        self.commandbar = None;

        self.hintbar.set_needs_redraw(true);
    }

    fn handle_quit(&mut self) {
        if !self.view.get_current_status().modified ||
            self.view.get_current_status().file_name.as_deref().unwrap() == "UNTITLED" {
            self.hintbar.update_hint("[ Quitting ]");
            self.should_quit = true;
        } else if self.view.get_current_status().modified {
            self.hintbar.update_hint("[ Error quitting. The file has unsaved changes ]");
        }
    }

    fn handle_save(&mut self) {
        if self.view.is_file_loaded() {
            self.save_file(None);
        } else {
            self.show_command_prompt();
        }
    }

    fn save_file(&mut self, file_name: Option<&str>) {
        let result = if let Some(name) = file_name {
            self.view.save_as(name)
        } else {
            self.view.save()
        };

        if result.is_ok() {
            self.hintbar.update_hint("[ Successfully saved the file ]");
        } else {
            self.hintbar.update_hint("[ Error saving the file ]");
        }
    }

    pub fn update_status(&mut self) {
        let status = self.view.get_current_status();
        let title = format!(
            "Rsedit - {}",
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
            if let Some(cmdbar) = &mut self.commandbar {
                cmdbar.render(
                    self.terminal_size.height.saturating_sub(1)
                );
            } else {
                self.hintbar.render(
                    self.terminal_size.height.saturating_sub(1)
                );
            }
        }

        if self.terminal_size.height > 2 {
            self.view.render(0);
        }

        let new_cursor_position;

        if let Some(cmdbar) = &self.commandbar {
            new_cursor_position = Position {
                column: cmdbar.get_cursor_column(),
                row: self.terminal_size.height.saturating_sub(1),
            };
        } else {
            new_cursor_position = self.view.get_cursor_position();
        }

        Terminal::move_cursor_to(new_cursor_position);
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

        if let Some(cmdbar) = &mut self.commandbar {
            cmdbar.resize(
                Size {
                    width: new_size.width,
                    height: 1,
                }
            );
        }
    }
}