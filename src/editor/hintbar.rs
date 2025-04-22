use std::io::Error;

use super::{
    terminal::{
        Terminal,
        Size,
    },
    uielements::UIElement,
};

#[derive(Default)]
pub struct HintBar {
    current_hint: String,
    needs_redraw: bool,
}

impl HintBar {
    pub fn update_hint(&mut self, new_hint: String) {
        if new_hint != self.current_hint {
            self.current_hint = String::from(
                format!(
                    "[ HINT ] :: {}",
                    new_hint
                )
            );

            self.needs_redraw = true;
        }
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

    // fn get_size(&mut self) {
    //     return self.size;
    // }

    fn draw(&mut self, row: usize) -> Result<(), Error> {
        return Terminal::print_line(
            row,
            &self.current_hint
        );
    }
}
