mod buffer;
mod fileinfo;
mod location;
mod searchinfo;
mod searchdirection;

use std::{
    cmp::min,
    io::Error,
};
use super::{
    super::{
        commands::{
            Edit,
            Move,
        },
        Terminal,
        Size,
        Position,
        Row,
        Column,
        FileStatus,
        VERSION,
        Line,
    },
    UIElement,
};
use buffer::Buffer;
use fileinfo::FileInfo;
use location::Location;
use searchinfo::SearchInfo;
use searchdirection::SearchDirection;

#[derive(Default)]
pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    text_location: Location,
    scroll_offset: Position,
    search_info: Option<SearchInfo>,
}

impl View {
    pub fn get_current_status(&self) -> FileStatus {
        return FileStatus {
            lines_count: self.buffer.height(),
            current_line_index: self.text_location.line_index,
            modified: self.buffer.modified,
            file_name: format!(
                "{}",
                self.buffer.file_info,
            ).into(),
        };
    }

    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Edit::InsertCharacter(character) => {
                self.insert_char(character);
            },
            Edit::InsertTab => {
                self.insert_tab();
            }
            Edit::InsertLine => {
                self.insert_line();
            },
            Edit::DeletePrevious => {
                self.delete_previous();
            },
            Edit::DeleteNext => {
                self.delete_next();
            },
        }
    }

    pub fn handle_move_command(&mut self, command: Move) {
        let Size {
            width: _,
            height
        } = self.size;

        match command {
            Move::Up => {
                self.move_up(1);
            },
            Move::Down => {
                self.move_down(1);
            },
            Move::Left => {
                self.move_left();
            },
            Move::Right => {
                self.move_right();
            },
            Move::PageUp => {
                self.move_up(height.saturating_sub(1));
            },
            Move::PageDown => {
                self.move_down(height.saturating_sub(1));
            },
            Move::StartOfLine => {
                self.move_to_start_of_line();
            },
            Move::EndOfLine => {
                self.move_to_end_of_line();
            },
        }

        self.scroll_text_location_into_view();
    }

    pub fn load(&mut self, file: &str) -> Result<(), Error> {
        self.buffer = Buffer::load(file)?;
        self.set_needs_redraw(true);

        return Ok(());
    }

    pub const fn is_file_loaded(&self) -> bool {
        return self.buffer.is_file_loaded();
    }

    pub fn save(&mut self) -> Result<(), Error> {
        return self.buffer.save();
    }

    pub fn save_as(&mut self, file_name: &str) -> Result<(), Error> {
        return self.buffer.save_as(file_name);
    }

    pub fn get_cursor_position(&self) -> Position {
        return self.text_location_to_position().saturating_sub(self.scroll_offset);
    }

    fn center_text_location(&mut self) {
        let Size {
            width,
            height
        } = self.size;
        let Position {
            column,
            row
        } = self.text_location_to_position();

        let horizontal_mid = width.div_ceil(2);
        let vertical_mid = height.div_ceil(2);

        self.scroll_offset.column = column.saturating_sub(horizontal_mid);
        self.scroll_offset.row = row.saturating_sub(vertical_mid);

        self.set_needs_redraw(true);
    }

    fn text_location_to_position(&self) -> Position {
        let column = self
            .buffer
            .lines
            .get(
                self.text_location.line_index
            )
            .map_or(
                0,
                |line| {
                    return line.width_until(self.text_location.grapheme_index);
                }
            );
        let row = self.text_location.line_index;

        return Position {
            column,
            row,
        };
    }

    fn move_up(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }

    fn move_down(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }

    fn move_right(&mut self) {
        let line_width = self.buffer.lines.get(self.text_location.line_index).map_or(0, Line::grapheme_count);
        if self.text_location.grapheme_index < line_width {
            self.text_location.grapheme_index += 1;
        } else {
            self.move_to_start_of_line();
            self.move_down(1);
        }
    }

    fn move_left(&mut self) {
        if self.text_location.grapheme_index > 0 {
            self.text_location.grapheme_index -= 1;
        } else if self.text_location.line_index > 0 {
            self.move_up(1);
            self.move_to_end_of_line();
        }
    }

    fn move_to_start_of_line(&mut self) {
        self.text_location.grapheme_index = 0;
    }

    fn move_to_end_of_line(&mut self) {
        self.text_location.grapheme_index = self.buffer.lines.get(self.text_location.line_index).map_or(0, Line::grapheme_count);
    }

    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_index = self.buffer.lines.get(self.text_location.line_index).map_or(0, |line| { min(line.grapheme_count(), self.text_location.grapheme_index) });
    }

    fn snap_to_valid_line(&mut self) {
        self.text_location.line_index = min(self.text_location.line_index, self.buffer.height());
    }

    fn scroll_horizontally(&mut self, to_where: Column) {
        let Size {
            width,
            height: _
        } = self.size;
        let offset_changed = if to_where < self.scroll_offset.column {
            self.scroll_offset.column = to_where;

            true
        } else if to_where >= self.scroll_offset.column.saturating_add(width) {
            self.scroll_offset.column = to_where.saturating_sub(width).saturating_add(1);

            true
        } else {
            false
        };

        if offset_changed {
            self.set_needs_redraw(true);
        }
    }

    fn scroll_vertically(&mut self, to_where: Row) {
        let Size {
            width: _,
            height
        } = self.size;
        let offset_changed = if to_where < self.scroll_offset.row {
            self.scroll_offset.row = to_where;

            true
        } else if to_where >= self.scroll_offset.row.saturating_add(height) {
            self.scroll_offset.row = to_where.saturating_sub(height).saturating_add(1);

            true
        } else {
            false
        };

        if offset_changed {
            self.set_needs_redraw(true);
        }
    }

    fn scroll_text_location_into_view(&mut self) {
        let Position {
            column,
            row
        } = self.text_location_to_position();

        self.scroll_horizontally(column);
        self.scroll_vertically(row);
    }

    fn insert_char(&mut self, character: char) {
        let old_length = self.buffer.lines.get(self.text_location.line_index).map_or(0, Line::grapheme_count);

        self.buffer.insert_char(
            character,
            self.text_location
        );

        let new_length = self.buffer.lines.get(self.text_location.line_index).map_or(0, Line::grapheme_count);

        let grapheme_delta = new_length.saturating_sub(old_length);

        if grapheme_delta > 0 {
            self.handle_move_command(Move::Right);
        }

        self.set_needs_redraw(true);
    }

    fn insert_tab(&mut self) {
        for _ in 0..4 {
            self.insert_char(' ');
        }
    }

    fn insert_line(&mut self) {
        self.buffer.insert_line(self.text_location);
        self.handle_move_command(Move::Right);

        self.set_needs_redraw(true);
    }

    fn delete_previous(&mut self) {
        if self.text_location.line_index != 0 || self.text_location.grapheme_index != 0 {
            self.handle_move_command(Move::Left);
            self.buffer.remove_char(self.text_location);

            self.set_needs_redraw(true);
        }
    }

    fn delete_next(&mut self) {
        self.buffer.remove_char(self.text_location);

        self.set_needs_redraw(true);
    }

    pub fn enter_search(&mut self) {
        self.search_info = Some(
            SearchInfo {
                previous_location: self.text_location,
                previous_scroll_offset: self.scroll_offset,
                query: None,
            }
        );
    }

    pub fn exit_search(&mut self) {
        self.search_info = None;

        self.set_needs_redraw(true);
    }

    pub fn dismiss_search(&mut self) {
        if let Some(search_info) = &self.search_info {
            self.text_location = search_info.previous_location;
            self.scroll_offset = search_info.previous_scroll_offset;

            self.scroll_text_location_into_view();
        }

        self.search_info = None;

        self.set_needs_redraw(true);
    }

    pub fn search(&mut self, query: &str) {
        if let Some(search_info) = &mut self.search_info {
            search_info.query = Some(Line::from(query));
        }

        self.search_in_direction(
            self.text_location,
            SearchDirection::default(),
        );

        self.set_needs_redraw(true);
    }

    fn get_search_query(&self) -> Option<&Line> {
        let query = self.search_info
            .as_ref()
            .and_then(
                |search_info| {
                    return search_info.query.as_ref();
                }
            );

        return query;
    }

    fn search_in_direction(&mut self, from: Location, direction: SearchDirection) {
        if let Some(location) = self
            .get_search_query()
            .and_then(
                |query| {
                    if query.is_empty() {
                        return None;
                    } else if direction == SearchDirection::Forward {
                        return self.buffer.search_next(query, from);
                    } else {
                        return self.buffer.search_previous(query, from);
                    }
                }
            ) {
            self.text_location = location;
            self.center_text_location();
        };
    }

    pub fn search_next(&mut self) {
        let next_step = self
            .get_search_query()
            .map_or(
                1,
                |query| {
                    return min(
                        query.grapheme_count(),
                        1,
                    );
                }
            );
        let location = Location {
            line_index: self.text_location.line_index,
            grapheme_index: self.text_location.grapheme_index.saturating_add(next_step),
        };

        self.search_in_direction(location, SearchDirection::Forward);
    }

    pub fn search_previous(&mut self) {
        self.search_in_direction(self.text_location, SearchDirection::Backward);
    }

    fn render_line(line_number: usize, data: &str) -> Result<(), Error> {
        return Terminal::print_line(line_number, data);
    }

    fn render_welcome(width: usize) -> String {
        if width == 0 {
            return String::new();
        }

        let remaining_width = width.saturating_sub(1);

        let welcome_msg = format!("Welcome to Rsedit v{VERSION}!");
        let length = welcome_msg.len();

        if remaining_width < length {
            return String::from("~");
        }

        return format!(
            "{:<1}{:^remaining_width$}",
            "~",
            welcome_msg,
        );
    }
}

impl UIElement for View {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn get_needs_redraw(&self) -> bool {
        return self.needs_redraw;
    }

    fn set_size(&mut self, new_size: Size) {
        self.size = new_size;
    }

    // fn get_size(&mut self) -> Size {
    //     return self.size;
    // }

    fn draw(&mut self, row: usize) -> Result<(), Error> {
        let Size {
            width,
            height
        } = self.size;
        let final_row = row.saturating_add(height);

        for current_line in row..final_row {
            let line_index = current_line
                .saturating_sub(row)
                .saturating_add(self.scroll_offset.row);

            if let Some(line) = self.buffer.lines.get(line_index) {
                let left = self.scroll_offset.column;
                let right = self.scroll_offset.column.saturating_add(width);

                let query = self
                    .search_info
                    .as_ref()
                    .and_then(
                        |search_info| {
                            return search_info.query.as_deref();
                        }
                    );
                let selected_match = (self.text_location.line_index == line_index && query.is_some()).then_some(self.text_location.grapheme_index);

                Terminal::print_annotated_line(
                    current_line,
                    &line.get_annotated_visible_substr(left..right, query, selected_match),
                )?;
            } else if current_line == height.div_ceil(3) && self.buffer.is_empty() {
                Self::render_line(
                    current_line,
                    &Self::render_welcome(width)
                )?;
            } else {
                Self::render_line(
                    current_line,
                    "~"
                )?;
            }
        }

        return Ok(());
    }
}