mod buffer;
mod location;
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
use location::Location;
use line::Line;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    location: Location,
    scroll_offset: Location,
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
        let top = self.scroll_offset.y;

        for current_line in 0..height {
            if let Some(line) = self.buffer.lines.get(current_line.saturating_add(top)) {
                let left = self.scroll_offset.x;
                let right = self.scroll_offset.x.saturating_add(width);

                Self::render_line(
                    current_line,
                    &line.get(left..right)
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

    pub fn handle_command(&mut self, c: EditorCmd) {
        match c {
            EditorCmd::Resize(size) => {
                self.resize(size)
            },
            EditorCmd::Move(direction) => {
                self.move_text_location(&direction)
            },
            EditorCmd::Quit => {}
        }
    }

    pub fn load(&mut self, file: &str) {
        if let Ok(buffer) = Buffer::load(file) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    pub fn get_position(&self) -> Position {
        return self.location
            .subtract(&self.scroll_offset)
            .into();
    }

    fn move_text_location(&mut self, direction: &Direction) {
        let Location {
            mut x,
            mut y
        } = self.location;
        let Size {
            width,
            height
        } = self.size;

        match direction {
            Direction::Up => {
                y = y.saturating_sub(1);
            },
            Direction::Down => {
                y = y.saturating_add(1);
            },
            Direction::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    x = self.buffer.lines.get(y).map_or(0, Line::length);
                }
            },
            Direction::Right => {
                if x < self.buffer.lines.get(y).map_or(0, Line::length) {
                    x += 1;
                } else {
                    y = y.saturating_add(1);
                    x = 0;
                }
            },
            Direction::PageUp => {
                y = y.saturating_sub(height).saturating_sub(1);
            },
            Direction::PageDown => {
                y = y.saturating_add(height).saturating_sub(1);
            },
            Direction::Home => {
                x = 0;
            },
            Direction::End => {
                x = self.buffer.lines.get(y).map_or(0, Line::length);
            }
        }

        x = self.buffer.lines.get(y).map_or(0, |line| min(line.length(), x));
        y = min(y, self.buffer.lines.len());

        self.location = Location {
            x,
            y
        };
        self.scroll_location_into_view();
    }

    fn resize(&mut self, new_size: Size) {
        self.size = new_size;
        self.scroll_location_into_view();
        self.needs_redraw = true;
    }

    fn scroll_location_into_view(&mut self) {
        let Location {
            x,
            y
        } = self.location;
        let Size {
            width,
            height
        } = self.size;
        let mut offset_changed = false;

        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y
                .saturating_sub(height)
                .saturating_add(1);
            offset_changed = true;
        }

        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x
                .saturating_sub(width)
                .saturating_add(1);
            offset_changed = true;
        }

        self.needs_redraw = offset_changed;
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
            location: Location::default(),
            scroll_offset: Location::default(),
        };
    }
}

