use std::{
    io::Error,
    fs::read_to_string,
};

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load(file: &str) -> Result<Self, Error> {
        let data = read_to_string(file)?;
        let mut lines = Vec::new();

        for line in data.lines() {
            lines.push(String::from(line));
        }

        return Ok(Self { lines });
    }

    pub fn is_empty(&self) -> bool {
        return self.lines.is_empty();
    }
}

