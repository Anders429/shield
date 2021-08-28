use crate::{components, events::{Events, Input}, World};

pub(crate) fn restart<const ENTITY_COUNT: usize>(world: &mut World<ENTITY_COUNT>, input: Input) -> Events {
    if input.has_a() {
        *world = World::default();
        world.new_game();
    }

    Events::default()
}
