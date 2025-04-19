use std::{
    ops::Range,
    cmp::min
};

pub struct Line {
    string: String,
}

impl Line {
    pub fn from(s: &str) -> Self {
        Self {
            string: String::from(s),
        }
    }

    pub fn get(&self, range: Range<usize>) -> String {
        let start = range.start;
        let end = min(range.end, self.string.len());

        return String::from(self.string.get(start..end).unwrap_or_default());
    }
}

