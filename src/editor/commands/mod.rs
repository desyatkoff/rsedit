mod editcmd;
mod movecmd;
mod systemcmd;

use std::convert::TryFrom;
use crossterm::event::Event;
pub use editcmd::Edit;
pub use movecmd::Move;
pub use systemcmd::System;
use super::Size;

#[derive(Clone, Copy)]
pub enum Command {
    Move(Move),
    Edit(Edit),
    System(System),
}

impl TryFrom<Event> for Command {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(key_event) => {
                return Edit::try_from(key_event)
                    .map(
                        Command::Edit
                    )
                    .or_else(
                        |_| Move::try_from(key_event)
                            .map(Command::Move)
                    )
                    .or_else(
                        |_| System::try_from(key_event)
                        .map(Command::System)
                    );
            },
            Event::Resize(width_u16, height_u16) => {
                return Ok(
                    Self::System(
                        System::Resize(
                            Size {
                                width: width_u16 as usize,
                                height: height_u16 as usize,
                            }
                        )
                    )
                );
            },
            _ => {
                return Err(String::new());
            }
        }
    }
}
