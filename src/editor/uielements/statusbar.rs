use std::io::Error;
use super::{
    super::{
        Terminal,
        Size,
        FileStatus,
    },
    UIElement,
};

#[derive(Default)]
pub struct StatusBar {
    current_status: FileStatus,
    size: Size,
    needs_redraw: bool,
}

impl StatusBar {
    pub fn update_status(&mut self, new_status: FileStatus) {
        if new_status != self.current_status {
            self.current_status = new_status;

            self.set_needs_redraw(true);
        }
    }
}

impl UIElement for StatusBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn get_needs_redraw(&self) -> bool {
        return self.needs_redraw;
    }

    fn set_size(&mut self, new_size: Size) {
        self.size = new_size;
    }

    fn draw(&mut self, row: usize) -> Result<(), Error> {
        let file_name = self.current_status.file_name.as_deref().unwrap();
        let lines_count = self.current_status.lines_count_to_string();
        let modified_indicator = self.current_status.modified_indicator_to_string();
        let position_indicator = self.current_status.position_indicator_to_string();
        let left = format!(
            "[ STATUS ] :: [ {} ] [ {} ]",
            file_name,
            modified_indicator,
        );
        let right = format!(
            " [ {} ] [ {} ]",
            position_indicator,
            lines_count,
        );
        let remainder_length = self.size.width.saturating_sub(left.len());
        let final_status = format!(
            "{left}{right:>remainder_length$}"
        );
        let to_print = if final_status.len() <= self.size.width {
            final_status
        } else {
            String::new()
        };

        Terminal::print_inverted_line(
            row,
            &to_print,
        )?;

        return Ok(());
    }
}