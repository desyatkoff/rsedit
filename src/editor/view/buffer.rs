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
        if at_where.line_index > self.height() {
            return;
        }

        if at_where.line_index == self.height() {
            self.lines.push(
                Line::from(
                    &String::from(character)
                )
            );
        } else if let Some(line) = self.lines.get_mut(at_where.line_index) {
            line.insert_char(character, at_where.grapheme_index);
        }
    }

    pub fn insert_line(&mut self, at_where: Location) {
        if at_where.line_index == self.height() {
            self.lines.push(Line::default());
        } else if let Some(line) = self.lines.get_mut(at_where.line_index) {
            let new_line = line.split(at_where.grapheme_index);

            self.lines.insert(
                at_where.line_index.saturating_add(1),
                new_line
            );
        }
    }

    pub fn remove_char(&mut self, at_where: Location) {
        if let Some(line) = self.lines.get(at_where.line_index) {
            if at_where.grapheme_index >= line.grapheme_count() && self.height() > at_where.line_index.saturating_add(1) {
                let next_line = self.lines.remove(
                    at_where.line_index.saturating_add(1)
                );

                self.lines[at_where.line_index].append(&next_line);
            } else if at_where.grapheme_index < line.grapheme_count() {
                self.lines[at_where.line_index].remove_char(at_where.grapheme_index);
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.lines.is_empty();
    }

    pub fn height(&self) -> usize {
        return self.lines.len();
    }
}

