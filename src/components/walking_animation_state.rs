#[derive(Copy, Clone)]
pub(crate) enum WalkingAnimationState {
    StandingA,
    StepA,
    StandingB,
    StepB,
}

impl Default for WalkingAnimationState {
    fn default() -> Self {
        Self::StandingA
    }
}

impl WalkingAnimationState {
    pub(crate) fn to_index(&self) -> usize {
        match self {
            WalkingAnimationState::StandingA | WalkingAnimationState::StandingB => 0,
            WalkingAnimationState::StepA => 1,
            WalkingAnimationState::StepB => 2,
        }
    }
}
