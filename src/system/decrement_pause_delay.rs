use crate::{components, events::Events, World};

pub(crate) fn decrement_pause_delay<const ENTITY_COUNT: usize>(world: &mut World<ENTITY_COUNT>) -> Events {
    world.resources.pause_delay = world.resources.pause_delay.saturating_sub(1);

    Events::default()
}