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
    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_redraw {
            return Ok(());
        }

        let Size {
            width,
            height,
        } = self.size;

        if width == 0 || height == 0 {
            return Ok(());
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
                )?;
            } else if current_line == vertical_center && self.buffer.is_empty() {
                Self::render_line(
                    current_line,
                    &Self::render_welcome(width)
                )?;
            } else {
                Self::render_line(
                    current_line,
                    "~"
                )?;
            }
        }

        self.needs_redraw = false;

        return Ok(());
    }

    fn render_line(line_number: usize, data: &str) -> Result<(), Error> {
        Terminal::move_cursor_to(
            Position {
                row: line_number,
                column: 0
            }
        )?;
        Terminal::clear_line()?;
        Terminal::print(data)?;

        return Ok(());
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


// impl View {
//     pub fn render(&self) -> Result<(), Error> {
//         if self.buffer.is_empty() {
//             Self::render_welcome()?;
//         } else {
//             self.render_buffer()?;
//         }

//         return Ok(());
//     }

//     pub fn render_welcome() -> Result<(), Error> {
//         let Size {
//             width: _,
//             height
//         } = Terminal::size()?;

//         for current_line in 0..height {
//             Terminal::clear_line()?;

//             if current_line == height / 3 {
//                 Self::draw_welcome_msg()?;
//             } else {
//                 Self::draw_empty_line()?;
//             }

//             if current_line.saturating_add(1) < height {
//                 Terminal::print("\r\n")?;
//             }
//         }

//         return Ok(());
//     }

//     pub fn render_buffer(&self) -> Result<(), Error> {
//         let Size {
//             width: _,
//             height
//         } = Terminal::size()?;

//         for current_line in 0..height {
//             Terminal::clear_line()?;

//             if let Some(line) = self.buffer.lines.get(current_line) {
//                 Terminal::print(&format!("{line}\r\n"))?;
//             } else {
//                 Self::draw_empty_line()?;
//             }
//         }

//         return Ok(());
//     }

//     pub fn load(&mut self, file: &str) {
//         if let Ok(buffer) = Buffer::load(file) {
//             self.buffer = buffer;
//         }
//     }

//     fn draw_welcome_msg() -> Result<(), Error> {
//         let mut welcome_msg = format!("Welcome to the Rsedit v{VERSION}!");
//         let width = Terminal::size()?.width;
//         let length = welcome_msg.len();
//         let padding = (width.saturating_sub(length)) / 2;
//         let spaces = " ".repeat(padding.saturating_sub(1));

//         welcome_msg = format!("~{spaces}{welcome_msg}");
//         welcome_msg.truncate(width);

//         Terminal::print(&welcome_msg)?;

//         return Ok(());
//     }

//     fn draw_empty_line() -> Result<(), Error> {
//         Terminal::print("~\r\n")?;

//         return Ok(());
//     }
// }

