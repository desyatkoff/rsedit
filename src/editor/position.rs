#[derive(Copy, Clone, Default)]
pub struct Position {
    pub column: usize,
    pub row: usize,
}

impl Position {
    pub const fn saturating_sub(self, other: Self) -> Self {
        return Self {
            column: self.column.saturating_sub(other.column),
            row: self.row.saturating_sub(other.row),
        };
    }
}