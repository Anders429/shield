use crate::{components, events::Events, World};
use itertools::izip;

pub(crate) fn toggle_walking_animation_state<const ENTITY_COUNT: usize>(
    world: &mut World<ENTITY_COUNT>,
) -> Events {
    for (entity, walking_timer, walking_animation_state) in izip!(
        world.entities.iter(),
        world.components.walking_timers.iter_mut(),
        world.components.walking_animation_states.iter_mut()
    ) {
        if entity.is_walking() && entity.has_walking_timer() && entity.has_walking_animation_state()
        {
            if *walking_timer == 0 {
                *walking_animation_state = match walking_animation_state {
                    components::WalkingAnimationState::StandingA => {
                        components::WalkingAnimationState::StepA
                    }
                    components::WalkingAnimationState::StepA => {
                        components::WalkingAnimationState::StandingB
                    }
                    components::WalkingAnimationState::StandingB => {
                        components::WalkingAnimationState::StepB
                    }
                    components::WalkingAnimationState::StepB => {
                        components::WalkingAnimationState::StandingA
                    }
                };
                *walking_timer = 10;
            } else {
                *walking_timer -= 1;
            }
        }
    }

    Events::default()
}
