#[derive(Clone, Copy, PartialEq)]
pub(crate) enum AcceptsInput {
    None,
    Player,
    FollowsPlayer,
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

    pub(crate) fn follows_player(&self) -> bool {
        matches!(self, AcceptsInput::FollowsPlayer)
    }
}
