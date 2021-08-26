#[derive(Clone, Copy)]
pub(crate) enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Up
    }
}
