mod attribute;

use std::io::{
    stdout,
    Write,
    Error,
};
use crossterm::{
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        size,
        Clear,
        ClearType,
        EnterAlternateScreen,
        LeaveAlternateScreen,
        EnableLineWrap,
        DisableLineWrap,
        SetTitle
    },
    cursor::{
        MoveTo,
        Hide,
        Show,
    },
    style::{
        Attribute::{
            Reset,
            Reverse,
        },
        Print,
        ResetColor,
        SetBackgroundColor,
        SetForegroundColor,
    },
    queue,
    Command,
};
use attribute::Attribute;
use super::{
    Position,
    Size,
    AnnotatedString,
};

pub struct Terminal;

impl Terminal {
    pub fn init() -> Result<(), Error> {
        enable_raw_mode()?;

        Self::enter_altscreen()?;
        Self::disable_line_wrap()?;
        Self::clear_all()?;
        Self::move_cursor_to(
            Position {
                column: 0,
                row: 0,
            }
        )?;
        Self::execute()?;

        return Ok(());
    }

    pub fn kill() -> Result<(), Error> {
        Self::leave_altscreen()?;
        Self::enable_line_wrap()?;
        Self::show_cursor()?;
        Self::execute()?;

        disable_raw_mode()?;

        return Ok(());
    }

    pub fn enter_altscreen() -> Result<(), Error> {
        Self::queue_cmd(EnterAlternateScreen)?;

        return Ok(());
    }

    pub fn leave_altscreen() -> Result<(), Error> {
        Self::queue_cmd(LeaveAlternateScreen)?;

        return Ok(());
    }

    pub fn clear_all() -> Result<(), Error> {
        Self::queue_cmd(
            Clear(ClearType::All),
        )?;

        return Ok(());
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_cmd(
            Clear(ClearType::CurrentLine),
        )?;

        return Ok(());
    }

    pub fn move_cursor_to(pos: Position) -> Result<(), Error> {
        Self::queue_cmd(
            MoveTo(
                pos.column as u16,
                pos.row as u16
            )
        )?;

        return Ok(());
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_cmd(Hide)?;

        return Ok(());
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_cmd(Show)?;

        return Ok(());
    }

    pub fn enable_line_wrap() -> Result<(), Error> {
        Self::queue_cmd(EnableLineWrap)?;

        return Ok(());
    }

    pub fn disable_line_wrap() -> Result<(), Error> {
        Self::queue_cmd(DisableLineWrap)?;

        return Ok(());
    }

    pub fn set_title(new_title: &str) -> Result<(), Error> {
        Self::queue_cmd(SetTitle(new_title))?;

        return Ok(());
    }

    pub fn print(s: &str) -> Result<(), Error> {
        Self::queue_cmd(Print(s))?;

        return Ok(());
    }

    pub fn print_line(line_number: usize, data: &str) -> Result<(), Error> {
        Self::move_cursor_to(
            Position {
                row: line_number,
                column: 0,
            }
        )?;
        Self::clear_line()?;
        Self::print(data)?;

        return Ok(());
    }

    pub fn print_annotated_line(line_number: usize, annotated_string: &AnnotatedString) -> Result<(), Error> {
        Self::move_cursor_to(
            Position {
                column: 0,
                row: line_number,
            }
        )?;

        Self::clear_line()?;

        annotated_string
            .into_iter()
            .try_for_each(
                |part| -> Result<(), Error> {
                    if let Some(annotation_type) = part.annotation_type {
                        let attribute: Attribute = annotation_type.into();

                        Self::set_attribute(&attribute)?;
                    }

                    Self::print(part.string)?;
                    Self::reset_color()?;

                    return Ok(());
                }
            )?;

        return Ok(());
    }

    fn set_attribute(attribute: &Attribute) -> Result<(), Error> {
        if let Some(foreground_color) = attribute.foreground {
            Self::queue_cmd(SetForegroundColor(foreground_color))?;
        }

        if let Some(background_color) = attribute.background {
            Self::queue_cmd(SetBackgroundColor(background_color))?;
        }

        return Ok(());
    }

    fn reset_color() -> Result<(), Error> {
        Self::queue_cmd(ResetColor)?;

        return Ok(());
    }

    pub fn print_inverted_line(line_number: usize, data: &str) -> Result<(), Error> {
        let width = Self::size()?.width;

        return Self::print_line(
            line_number,
            &format!(
                "{Reverse}{data:width$.width$}{Reset}"
            ),
        )
    }

    pub fn size() -> Result<Size, Error> {
        let (
            width_u16,
            height_u16
        ) = size()?;

        return Ok(
            Size {
                width: width_u16 as usize,
                height: height_u16 as usize
            }
        );
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;

        return Ok(());
    }

    pub fn queue_cmd<T: Command>(cmd: T) -> Result<(), Error> {
        queue!(
            stdout(),
            cmd
        )?;

        return Ok(());
    }
}