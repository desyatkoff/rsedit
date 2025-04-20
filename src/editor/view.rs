mod buffer;
mod line;

use std::cmp::min;
use super::{
    terminal::{
        Terminal,
        Size,
        Position,
    },
    commands::{
        Direction,
        EditorCmd,
    },
};
use buffer::Buffer;
use line::Line;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default, Copy, Clone)]
pub struct Location {
    pub grapheme_index: usize,
    pub line_index: usize,
}

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    text_location: Location,
    scroll_offset: Position,
}

impl View {
    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }

        let Size {
            width,
            height
        } = self.size;

        if width == 0 || height == 0 {
            return;
        }

        let vertical_center = height / 3;
        let top = self.scroll_offset.row;

        for current_line in 0..height {
            if let Some(line) = self.buffer.lines.get(current_line.saturating_add(top)) {
                let left = self.scroll_offset.column;
                let right = self.scroll_offset.column.saturating_add(width);

                Self::render_line(
                    current_line,
                    &line.get_visible_graphemes(left..right)
                );
            } else if current_line == vertical_center && self.buffer.is_empty() {
                Self::render_line(
                    current_line,
                    &Self::render_welcome(width)
                );
            } else {
                Self::render_line(current_line, "~");
            }
        }

        self.needs_redraw = false;
    }

    pub fn handle_command(&mut self, command: EditorCmd) {
        match command {
            EditorCmd::Resize(size) => {
                self.resize(size);
            },
            EditorCmd::Move(direction) => {
                self.move_text_location(&direction);
            },
            EditorCmd::Insert(character) => {
                self.insert_char(character);
            },
            EditorCmd::DeleteLeft => {
                self.delete_left();
            },
            EditorCmd::DeleteRight => {
                self.delete_right();
            },
            EditorCmd::Quit => {},
        }
    }

    pub fn load(&mut self, file: &str) {
        if let Ok(buffer) = Buffer::load(file) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    pub fn get_cursor_position(&self) -> Position {
        return self.text_location_to_position().saturating_sub(self.scroll_offset);
    }

    fn text_location_to_position(&self) -> Position {
        let row = self.text_location.line_index;
        let column = self.buffer.lines.get(row).map_or(0, |line| {
            line.width_until(self.text_location.grapheme_index)
        });

        return Position {
            column,
            row
        };
    }

    fn move_text_location(&mut self, direction: &Direction) {
        let Size {
            width: _,
            height
        } = self.size;

        match direction {
            Direction::Up => {
                self.move_up(1);
            },
            Direction::Down => {
                self.move_down(1);
            },
            Direction::Left => {
                self.move_left();
            },
            Direction::Right => {
                self.move_right();
            },
            Direction::PageUp => {
                self.move_up(height.saturating_sub(1));
            },
            Direction::PageDown => {
                self.move_down(height.saturating_sub(1));
            },
            Direction::Home => {
                self.move_to_start_of_line();
            },
            Direction::End => {
                self.move_to_end_of_line();
            },
        }

        self.scroll_text_location_into_view();
    }

    fn move_up(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }

    fn move_down(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }

    fn move_right(&mut self) {
        let line_width = self.buffer.lines.get(self.text_location.line_index).map_or(0, Line::grapheme_count);
        if self.text_location.grapheme_index < line_width {
            self.text_location.grapheme_index += 1;
        } else {
            self.move_to_start_of_line();
            self.move_down(1);
        }
    }

    fn move_left(&mut self) {
        if self.text_location.grapheme_index > 0 {
            self.text_location.grapheme_index -= 1;
        } else if self.text_location.line_index > 0 {
            self.move_up(1);
            self.move_to_end_of_line();
        }
    }

    fn move_to_start_of_line(&mut self) {
        self.text_location.grapheme_index = 0;
    }

    fn move_to_end_of_line(&mut self) {
        self.text_location.grapheme_index = self.buffer.lines.get(self.text_location.line_index).map_or(0, Line::grapheme_count);
    }

    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_index = self.buffer.lines.get(self.text_location.line_index).map_or(0, |line| { min(line.grapheme_count(), self.text_location.grapheme_index) });
    }

    fn snap_to_valid_line(&mut self) {
        self.text_location.line_index = min(self.text_location.line_index, self.buffer.height());
    }

    fn resize(&mut self, new_size: Size) {
        self.size = new_size;
        self.scroll_text_location_into_view();
        self.needs_redraw = true;
    }

    fn scroll_horizontally(&mut self, to_where: usize) {
        let Size {
            width,
            height: _
        } = self.size;
        let offset_changed = if to_where < self.scroll_offset.column {
            self.scroll_offset.column = to_where;

            true
        } else if to_where >= self.scroll_offset.column.saturating_add(width) {
            self.scroll_offset.column = to_where.saturating_sub(width).saturating_add(1);

            true
        } else {
            false
        };

        self.needs_redraw = self.needs_redraw || offset_changed;
    }

    fn scroll_vertically(&mut self, to_where: usize) {
        let Size {
            width: _,
            height
        } = self.size;
        let offset_changed = if to_where < self.scroll_offset.row {
            self.scroll_offset.row = to_where;

            true
        } else if to_where >= self.scroll_offset.row.saturating_add(height) {
            self.scroll_offset.row = to_where.saturating_sub(height).saturating_add(1);

            true
        } else {
            false
        };

        self.needs_redraw = self.needs_redraw || offset_changed;
    }

    fn scroll_text_location_into_view(&mut self) {
        let Position {
            column,
            row
        } = self.text_location_to_position();

        self.scroll_horizontally(column);
        self.scroll_vertically(row);
    }

    fn insert_char(&mut self, character: char) {
        let old_length = self.buffer.lines.get(self.text_location.line_index).map_or(0, Line::grapheme_count);

        self.buffer.insert_char(
            character,
            self.text_location
        );

        let new_length = self.buffer.lines.get(self.text_location.line_index).map_or(0, Line::grapheme_count);

        let grapheme_delta = new_length.saturating_sub(old_length);

        if grapheme_delta > 0 {
            self.move_text_location(&Direction::Right);
        }

        self.needs_redraw = true;
    }

    fn delete_left(&mut self) {
        if self.text_location.line_index != 0 || self.text_location.grapheme_index != 0 {
            self.move_text_location(&Direction::Left);
            self.buffer.remove_char(self.text_location);

            self.needs_redraw = true;
        }
    }

    fn delete_right(&mut self) {
        self.buffer.remove_char(self.text_location);

        self.needs_redraw = true;
    }

    fn render_line(line_number: usize, data: &str) {
        Terminal::print_line(line_number, data);
    }

    fn render_welcome(width: usize) -> String {
        if width == 0 {
            return String::from(" ");
        }

        let welcome_msg = format!("Welcome to the Rsedit v{VERSION}!");
        let length = welcome_msg.len();

        if width <= length {
            return String::from("~");
        }

        let padding = (width.saturating_sub(length).saturating_sub(1)) / 2;

        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_msg);
        full_message.truncate(width);

        return full_message;
    }
}

impl Default for View {
    fn default() -> Self {
        return Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            text_location: Location::default(),
            scroll_offset: Position::default(),
        };
    }
}

