mod terminal;

use crossterm::{
    event::{
        read,
        Event,
        Event::Key,
        KeyCode::Char,
        KeyEvent,
        KeyModifiers
    }
};
use terminal::Terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn new() -> Self {
        return Self { should_quit: false };
    }

    pub fn run(&mut self) {
        Terminal::init().unwrap();

        let result = self.repl();

        Terminal::kill().unwrap();

        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }

            let event = read()?;

            self.eval_event(&event);
        }

        return Ok(());
    }

    fn eval_event(&mut self, event: &Event) {
        if let Key(
            KeyEvent {
                code,
                modifiers,
                ..
            }
        ) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                _ => {
                    return;
                }
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear()?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }

        return Ok(());
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let height = Terminal::size()?.1;

        for current_row in 0..height {
            print!("~");

            if current_row + 1 < height {
                print!("\r\n");
            }
        }

        return Ok(());
    }
}

