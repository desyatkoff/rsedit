use std::{
    io::Error,
    fs::read_to_string,
};
use super::{
    line::Line,
    Location,
};

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

    pub fn insert_char(&mut self, character: char, at_where: Location) {
        if at_where.line_index > self.lines.len() {
            return;
        }

        if at_where.line_index == self.lines.len() {
            self.lines.push(
                Line::from(
                    &String::from(character)
                )
            );
        } else if let Some(line) = self.lines.get_mut(at_where.line_index) {
            line.insert_char(character, at_where.grapheme_index);
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.lines.is_empty();
    }

    pub fn height(&self) -> usize {
        return self.lines.len();
    }
}

