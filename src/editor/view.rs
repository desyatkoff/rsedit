mod buffer;

use std::io::Error;
use super::terminal::{
    Terminal,
    Size,
    Position,
};
use buffer::Buffer;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl View {
    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }

        let Size {
            width,
            height,
        } = self.size;

        if width == 0 || height == 0 {
            return;
        }

        let vertical_center = height / 3;

        for current_line in 0..height {
            if let Some(line) = self.buffer.lines.get(current_line) {
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };

                Self::render_line(
                    current_line,
                    truncated_line
                );
            } else if current_line == vertical_center && self.buffer.is_empty() {
                Self::render_line(
                    current_line,
                    &Self::render_welcome(width)
                );
            } else {
                Self::render_line(
                    current_line,
                    "~"
                );
            }
        }

        self.needs_redraw = false;
    }

    fn render_line(line_number: usize, data: &str) {
        Terminal::print_line(
            line_number,
            data
        );
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

        let mut full_msg = format!("~{}{}", " ".repeat(padding), welcome_msg);

        full_msg.truncate(width);

        return full_msg;
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    pub fn resize(&mut self, new_size: Size) {
        self.size = new_size;
        self.needs_redraw = true;
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

