#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub enum SearchDirection {
    #[default]
    Forward,
    Backward,
}
