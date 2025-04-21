#[derive(Default, Eq, PartialEq, Debug)]
pub struct FileStatus {
    pub lines_count: usize,
    pub current_line_index: usize,
    pub modified: bool,
    pub file_name: Option<String>,
}

impl FileStatus {
    pub fn modified_indicator_to_string(&self) -> String {
        if self.modified {
            return String::from("MODIFIED");
        } else {
            return String::from("NOT MODIFIED");
        }
    }

    pub fn lines_count_to_string(&self) -> String {
        if self.lines_count != 1 {
            return format!(
                "{} LINES",
                self.lines_count
            );
        } else {
            return String::from("1 LINE");
        }
    }

    pub fn position_indicator_to_string(&self) -> String {
        format!(
            "{}:{}",
            self.current_line_index.saturating_add(1),
            self.lines_count,
        )
    }
}

