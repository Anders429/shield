#[derive(Copy, Clone)]
pub(crate) enum WalkingAnimationState {
    Standing,
    Step,
}

impl Default for WalkingAnimationState {
    fn default() -> Self {
        Self::Standing
    }
}
