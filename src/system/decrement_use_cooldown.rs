use crate::{components, events::Events, World};
use itertools::izip;

pub(crate) fn decrement_use_cooldown<const ENTITY_COUNT: usize>(
    world: &mut World<ENTITY_COUNT>,
) -> Events {
    for (entity, use_cooldown) in izip!(
        world.entities.iter_mut(),
        world.components.use_cooldowns.iter_mut()
    ) {
        if entity.has_use_cooldown() {
            if *use_cooldown == 0 {
                entity.remove_use_cooldown();
            } else {
                *use_cooldown -= 1;
            }
        }
    }

    Events::default()
}
