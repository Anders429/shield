#[derive(Clone, Copy)]
pub(crate) enum AcceptsInput {
    None,
    Player,
}

impl Default for AcceptsInput {
    fn default() -> Self {
        Self::None
    }
}

impl AcceptsInput {
    pub(crate) fn from_player(&self) -> bool {
        matches!(self, AcceptsInput::Player)
    }
}
