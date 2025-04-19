use std::{
    io,
    io::Read,
};
use crossterm::{
    terminal::enable_raw_mode,
    terminal::disable_raw_mode,
};

pub struct Editor{}

impl Editor {
    pub fn new() -> Self {
        return Editor{};
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();

        for byte in io::stdin().bytes() {
            let ch = byte.unwrap() as char;

            println!("{}", ch);

            if ch == 'q' {
                disable_raw_mode().unwrap();

                break;
            }
        }
    }
}

