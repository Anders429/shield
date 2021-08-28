#[derive(Clone, Copy)]
pub(crate) enum GameState {
    Playing,
    Pause,
    GameOver,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Playing
    }
}
