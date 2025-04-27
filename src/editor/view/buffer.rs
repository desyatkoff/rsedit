use std::{
    io::{
        Error,
        Write,
    },
    fs::{
        read_to_string,
        File,
    },
};
use super::{
    Line,
    Location,
    FileInfo,
};

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
    pub modified: bool,
    pub file_info: FileInfo,
}

impl Buffer {
    pub fn load(file: &str) -> Result<Self, Error> {
        let data = read_to_string(file)?;
        let mut lines = Vec::new();

        for line_data in data.lines() {
            lines.push(Line::from(line_data));
        }

        return Ok(
            Self {
                lines: lines,
                modified: false,
                file_info: FileInfo::from(file),
            }
        );
    }

    fn save_file(&self, file_info: &FileInfo) -> Result<(), Error> {
        if let Some(file_path) = &file_info.get_path() {
            let mut file = File::create(file_path)?;

            for line in &self.lines {
                writeln!(
                    file,
                    "{line}"
                )?;
            }
        }

        return Ok(());
    }

    pub fn save_as(&mut self, file_name: &str) -> Result<(), Error> {
        let file_info = FileInfo::from(file_name);

        self.save_file(&file_info)?;
        self.file_info = file_info;
        self.modified = false;

        return Ok(());
    }

    pub fn save(&mut self) -> Result<(), Error> {
        self.save_file(&self.file_info)?;
        self.modified = false;

        return Ok(());
    }

    pub fn insert_char(&mut self, character: char, at_where: Location) {
        if at_where.line_index == self.height() {
            self.lines.push(
                Line::from(
                    &String::from(character)
                )
            );

            self.modified = true;
        } else if let Some(line) = self.lines.get_mut(at_where.line_index) {
            line.insert_char(character, at_where.grapheme_index);

            self.modified = true;
        }
    }

    pub fn insert_line(&mut self, at_where: Location) {
        if at_where.line_index == self.height() {
            self.lines.push(Line::default());

            self.modified = true;
        } else if let Some(line) = self.lines.get_mut(at_where.line_index) {
            let new_line = line.split(at_where.grapheme_index);

            self.lines.insert(
                at_where.line_index.saturating_add(1),
                new_line
            );

            self.modified = true;
        }
    }

    pub fn remove_char(&mut self, at_where: Location) {
        if let Some(line) = self.lines.get(at_where.line_index) {
            if at_where.grapheme_index >= line.grapheme_count() && self.height() > at_where.line_index.saturating_add(1) {
                let next_line = self.lines.remove(
                    at_where.line_index.saturating_add(1)
                );

                self.lines[at_where.line_index].append(&next_line);

                self.modified = true;
            } else if at_where.grapheme_index < line.grapheme_count() {
                self.lines[at_where.line_index].remove_char(at_where.grapheme_index);

                self.modified = true;
            }
        }
    }

    pub fn search_next(&self, query: &str, from: Location) -> Option<Location> {
        if query.is_empty() {
            return None;
        }

        let mut is_first = true;

        for (line_index, line) in self
            .lines
            .iter()
            .enumerate()
            .cycle()
            .skip(
                from.line_index
            )
            .take(
                self
                    .lines
                    .len()
                    .saturating_add(1)
                ) {
            let from_grapheme_index = if is_first {
                is_first = false;
                from.grapheme_index
            } else {
                0
            };

            if let Some(grapheme_index) = line.search_next(query, from_grapheme_index) {
                return Some(
                    Location {
                        grapheme_index,
                        line_index,
                    }
                );
            }
        }

        return None;
    }

    pub fn search_previous(&self, query: &str, from: Location) -> Option<Location> {
        if query.is_empty() {
            return None;
        }

        let mut is_first = true;

        for (line_index, line) in self
            .lines
            .iter()
            .enumerate()
            .rev()
            .cycle()
            .skip(
                self
                    .lines
                    .len()
                    .saturating_sub(from.line_index)
                    .saturating_sub(1)
            )
            .take(
                self
                    .lines
                    .len()
                    .saturating_add(1)
            ) {
            let from_grapheme_index = if is_first {
                is_first = false;
                from.grapheme_index
            } else {
                line.grapheme_count()
            };

            if let Some(grapheme_index) = line.search_previous(query, from_grapheme_index) {
                return Some(
                    Location {
                        grapheme_index,
                        line_index,
                    }
                );
            }
        }

        return None;
    }

    pub fn is_empty(&self) -> bool {
        return self.lines.is_empty();
    }

    pub const fn is_file_loaded(&self) -> bool {
        return self.file_info.has_path();
    }

    pub fn height(&self) -> usize {
        return self.lines.len();
    }
}