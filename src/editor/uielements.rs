use std::io::Error;
use super::Size;

pub trait UIElement {
    fn set_needs_redraw(&mut self, value: bool);

    fn get_needs_redraw(&self) -> bool;

    fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.set_needs_redraw(true);
    }

    fn set_size(&mut self, size: Size);

    fn render(&mut self, row: usize) {
        if self.get_needs_redraw() {
            match self.draw(row) {
                Ok(()) => {
                    self.set_needs_redraw(false);
                },
                Err(_) => {},
            }
        }
    }

    fn draw(&mut self, row: usize) -> Result<(), Error>;
}
