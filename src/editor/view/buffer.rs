use std::{
    io::Error,
    fs::read_to_string,
};
use super::line::Line;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
}

impl Buffer {
    pub fn load(file: &str) -> Result<Self, Error> {
        let data = read_to_string(file)?;
        let mut lines = Vec::new();

        for line_data in data.lines() {
            lines.push(Line::from(line_data));
        }

        return Ok(Self { lines });
    }

    pub fn is_empty(&self) -> bool {
        return self.lines.is_empty();
    }
}

