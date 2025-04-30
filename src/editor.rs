mod terminal;
mod commands;
mod filestatus;
mod uielements;
mod line;
mod position;
mod size;
mod annotatedstring;

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
        Search,
    },
    Move::{
        Up,
        Down,
        Left,
        Right,
    },
    Edit::InsertLine,
};
use filestatus::FileStatus;
use line::Line;
use position::{
    Position,
    Row,
    Column,
};
use size::Size;
use annotatedstring::{
    AnnotatedString,
    AnnotationType,
};
use uielements::{
    UIElement,
    View,
    StatusBar,
    HintBar,
    CommandBar,
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default, Eq, PartialEq)]
enum PromptType {
    #[default]
    None,
    Search,
    Save,
}

impl PromptType {
    fn is_none(&self) -> bool {
        return *self == Self::None;
    }
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    statusbar: StatusBar,
    hintbar: HintBar,
    commandbar: CommandBar,
    prompt_type: PromptType,
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
        editor.handle_resize_command(Terminal::size().unwrap_or_default());
        editor.update_hint("[ Control + F -> Search ] [ Control + S -> Save ] [ Control + Q -> Quit ]");

        let args: Vec<String> = env::args().collect();

        if let Some(file) = args.get(1) {
            if editor.view.load(file).is_err() {
                editor.update_hint("[ Error opening the file ]");
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

            self.update_status();
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
        if let System(Resize(size)) = command {
            self.handle_resize_command(size);

            return;
        }

        match self.prompt_type {
            PromptType::Search => {
                self.process_search_command(command);
            },
            PromptType::Save => {
                self.process_save_command(command);
            },
            PromptType::None => {
                self.process_no_prompt_command(command);
            },
        }
    }

    fn process_search_command(&mut self, command: Command) {
        match command {
            System(Dismiss) => {
                self.set_prompt(PromptType::None);
                self.view.dismiss_search();
                self.update_hint("[ Cancelled searching ]");
            },
            Edit(InsertLine) => {
                self.set_prompt(PromptType::None);
                self.view.exit_search();
                self.update_hint("[ Done searching ]");
            },
            Edit(edit_command) => {
                self.commandbar.handle_edit_command(edit_command);

                let query = self.commandbar.get_value();

                self.view.search(&query);
            },
            Move(Up | Left) => {
                self.view.search_previous();
            },
            Move(Down | Right) => {
                self.view.search_next();
            },
            System(
                Quit
                | Resize(_)
                | Search
                | Save
            )
            | Move(_) => {},
        }
    }

    fn process_save_command(&mut self, command: Command) {
        match command {
            System(
                Quit
                | Resize(_)
                | Search
                | Save
            )
            | Move(_) => {},
            System(Dismiss) => {
                self.set_prompt(PromptType::None);
                self.update_hint("[ Cancelled saving ]");
            },
            Edit(InsertLine) => {
                let file_name = self.commandbar.get_value();
                self.save_file(Some(&file_name));
                self.set_prompt(PromptType::None);
            },
            Edit(edit_command) => {
                self.commandbar.handle_edit_command(edit_command);
            },
        }
    }

    fn process_no_prompt_command(&mut self, command: Command) {
        if matches!(command, System(Quit)) {
            self.handle_quit_command();
            return;
        }

        match command {
            System(
                Quit
                | Resize(_)
                | Dismiss
            ) => {},
            System(Search) => {
                self.set_prompt(PromptType::Search);
            },
            System(Save) => {
                self.handle_save_command();
            },
            Edit(edit_command) => {
                self.view.handle_edit_command(edit_command);
            },
            Move(move_command) => {
                self.view.handle_move_command(move_command);
            },
        }
    }

    fn handle_resize_command(&mut self, size: Size) {
        let bar_size = Size {
            width: size.width,
            height: 1,
        };

        self.terminal_size = size;
        self.view.resize(
            Size {
                width: size.width,
                height: size.height.saturating_sub(2),
            }
        );
        self.hintbar.resize(bar_size);
        self.statusbar.resize(bar_size);
        self.commandbar.resize(bar_size);
    }

    fn handle_quit_command(&mut self) {
        if !self.view.get_current_status().modified ||
            self.view.get_current_status().file_name.as_deref().unwrap() == "No file open" {
            self.update_hint("[ Quitting ]");
            self.should_quit = true;
        } else if self.view.get_current_status().modified {
            self.update_hint("[ Error quitting. The file has unsaved changes ]");
        }
    }

    fn handle_save_command(&mut self) {
        if self.view.is_file_loaded() {
            self.save_file(None);
        } else {
            self.set_prompt(PromptType::Save);
        }
    }

    fn save_file(&mut self, file_name: Option<&str>) {
        let result = if let Some(name) = file_name {
            self.view.save_as(name)
        } else {
            self.view.save()
        };

        if result.is_ok() {
            self.update_hint("[ Successfully saved the file ]");
        } else {
            self.update_hint("[ Error saving the file ]");
        }
    }

    fn set_prompt(&mut self, prompt_type: PromptType) {
        match prompt_type {
            PromptType::Search => {
                self.view.enter_search();
                self.commandbar.set_prompt("[ COMMAND ] :: Search: ");
            },
            PromptType::Save => {
                self.commandbar.set_prompt("[ COMMAND ] :: Save as: ");
            },
            PromptType::None => {
                self.hintbar.set_needs_redraw(true);
            },
        }

        self.commandbar.clear_value();
        self.prompt_type = prompt_type;
    }

    fn is_in_prompt(&self) -> bool {
        return !self.prompt_type.is_none();
    }

    fn update_hint(&mut self, new_hint: &str) {
        self.hintbar.update_hint(new_hint);
    }

    fn update_screen(&mut self) {
        if self.terminal_size.height == 0 || self.terminal_size.width == 0 {
            return;
        }

        Terminal::hide_cursor();

        self.statusbar.render(self.terminal_size.height.saturating_sub(2));

        if self.terminal_size.height > 1 {
            if self.is_in_prompt() {
                self.commandbar.render(self.terminal_size.height.saturating_sub(1));
            } else {
                self.hintbar.render(self.terminal_size.height.saturating_sub(1));
            }
        }

        if self.terminal_size.height > 2 {
            self.view.render(0);
        }

        let new_cursor_position = if self.is_in_prompt() {
            Position {
                column: self.commandbar.get_cursor_column(),
                row: self.terminal_size.height.saturating_sub(1),
            }
        } else {
            self.view.get_cursor_position()
        };

        Terminal::move_cursor_to(new_cursor_position);
        Terminal::show_cursor();
        Terminal::execute();
    }

    fn update_status(&mut self) {
        let status = self.view.get_current_status();
        let title = format!(
            "Rsedit - {}",
            status.file_name.as_deref().unwrap()
        );

        self.statusbar.update_status(status);

        if title != self.title && matches!(Terminal::set_title(&title), Ok(())) {
            self.title = title;
        }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        Terminal::kill();
    }
}