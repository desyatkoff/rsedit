use std::io::Error;
use super::{
    Terminal,
    Size,
    UIElement,
};

struct Hint {
    text: String,
}

impl Default for Hint {
    fn default() -> Self {
        return Self {
            text: String::from("[ Control + S -> Save ] [ Control + Q -> Quit ]"),
        };
    }
}

#[derive(Default)]
pub struct HintBar {
    current_hint: Hint,
    needs_redraw: bool,
}

impl HintBar {
    pub fn update_hint(&mut self, new_hint: &str) {
        self.current_hint = Hint {
            text: String::from(
                format!(
                    "[ HINT ] :: {}",
                    &new_hint
                )
            ),
        };

        self.set_needs_redraw(true);
    }
}

impl UIElement for HintBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn get_needs_redraw(&self) -> bool {
        return self.needs_redraw;
    }

    fn set_size(&mut self, new_size: Size) {
        // self.size = new_size;
    }

    fn draw(&mut self, row: usize) -> Result<(), Error> {
        return Terminal::print_line(
            row,
            &self.current_hint.text.clone()
        );
    }
}