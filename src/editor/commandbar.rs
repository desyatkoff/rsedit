use std::{
    cmp::min,
    io::Error
};
use super::{
    commands::Edit,
    uielements::UIElement,
    Line,
    Size,
    Terminal,
};

#[derive(Default)]
pub struct CommandBar {
    prompt: String,
    value: Line,
    size: Size,
    needs_redraw: bool,
}

impl CommandBar {
    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Edit::InsertCharacter(character) => {
                self.value.append_char(character)
            },
            Edit::DeletePrevious => {
                self.value.remove_last_char()
            },
            Edit::DeleteNext
            | Edit::InsertLine
            | Edit::InsertTab => {},
        }

        self.set_needs_redraw(true);
    }

    pub fn get_cursor_column(&self) -> usize {
        let max_width = self
            .prompt
            .len()
            .saturating_add(self.value.grapheme_count());

        return min(max_width, self.size.width);
    }

    pub fn clear_value(&mut self) {
        self.value = Line::default();
        self.set_needs_redraw(true);
    }

    pub fn get_value(&self) -> String {
        return format!("{}", self.value);
    }

    pub fn set_prompt(&mut self, prompt: &str) {
        self.prompt = String::from(prompt);
        self.set_needs_redraw(true);
    }
}

impl UIElement for CommandBar {
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
        let value_area = self.size.width.saturating_sub(self.prompt.len());
        let value_end = self.value.width();
        let value_start = value_end.saturating_sub(value_area);

        let message = format!(
            "{}{}",
            self.prompt,
            self.value.get_visible_graphemes(value_start..value_end)
        );

        let to_print = if message.len() <= self.size.width {
            message
        } else {
            String::new()
        };

        return Terminal::print_line(
            row,
            &to_print
        );
    }
}