use crate::editor::{
    Line,
    Position,
};
use super::Location;

pub struct SearchInfo {
    pub previous_location: Location,
    pub previous_scroll_offset: Position,
    pub query: Option<Line>,
}
