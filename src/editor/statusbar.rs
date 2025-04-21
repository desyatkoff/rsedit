use super::{
    terminal::{
        Terminal,
        Size,
    },
    FileStatus,
};

pub struct StatusBar {
    current_status: FileStatus,
    needs_redraw: bool,
    margin_bottom: usize,
    width: usize,
    row: usize,
    visible: bool,
}

impl StatusBar {
    pub fn new(margin_bottom: usize) -> Self {
        let size = Terminal::size().unwrap_or_default();
        let mut statusbar = Self {
            current_status: FileStatus::default(),
            needs_redraw: true,
            margin_bottom: margin_bottom,
            width: size.width,
            row: 0,
            visible: false,
        };

        statusbar.resize(size);

        return statusbar;
    }

    pub fn resize(&mut self, new_size: Size) {
        self.width = new_size.width;

        let mut row = 0;
        let mut visible = false;

        if let Some(result) = new_size
            .height
            .checked_sub(
                self.margin_bottom
            )
            .and_then(
                |result| result.checked_sub(1)
            ) {
            row = result;
            visible = true;
        }

        self.row = row;
        self.visible = visible;

        self.needs_redraw = true;
    }

    pub fn update_status(&mut self, new_status: FileStatus) {
        if new_status != self.current_status {
            self.current_status = new_status;
            self.needs_redraw = true;
        }
    }

    pub fn render(&mut self) {
        if !self.needs_redraw || !self.visible {
            return;
        }

        if let Ok(size) = Terminal::size() {
            let file_name = self.current_status.file_name.as_deref().unwrap();
            let lines_count = self.current_status.lines_count_to_string();
            let modified_indicator = self.current_status.modified_indicator_to_string();
            let position_indicator = self.current_status.position_indicator_to_string();
            let left = format!(
                "[ {} ] [ {} ]",
                file_name,
                modified_indicator,
            );
            let right = format!(
                "[ {} ] [ {} ]",
                position_indicator,
                lines_count,
            );
            let remainder_length = size.width.saturating_sub(left.len());
            let final_status = format!("{left}{right:>remainder_length$}");
            let to_print = if final_status.len() <= size.width {
                final_status
            } else {
                String::new()
            };

            let result = Terminal::print_inverted_line(
                self.row,
                &to_print,
            );

            self.needs_redraw = false;
        }
    }
}

