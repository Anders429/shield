use crate::{components, events::{Events, Input}, World};

pub(crate) fn unpause<const ENTITY_COUNT: usize>(world: &mut World<ENTITY_COUNT>, input: Input) -> Events {
    if input.has_start() && world.resources.pause_delay == 0 {
        world.resources.game_state = components::GameState::Playing;
        world.resources.pause_delay = 20;
    }

    Events::default()
}
