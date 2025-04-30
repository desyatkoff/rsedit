#[derive(Copy, Clone, Debug)]
pub enum GraphemeWidth {
    Half,
    Full,
}

impl From<GraphemeWidth> for usize {
    fn from(val: GraphemeWidth) -> Self {
        match val {
            GraphemeWidth::Half => {
                return 1;
            },
            GraphemeWidth::Full => {
                return 2;
            },
        }
    }
}
