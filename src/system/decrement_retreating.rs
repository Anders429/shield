use crate::{components, events::Events, World};
use itertools::izip;

pub(crate) fn decrement_retreating<const ENTITY_COUNT: usize>(world: &mut World<ENTITY_COUNT>) -> Events {
    for (entity, retreating) in izip!(world.entities.iter_mut(), world.components.retreatings.iter_mut()) {
        if entity.has_retreating() {
            if *retreating == 0 {
                entity.remove_retreating();
            } else {
                *retreating -= 1;
            }
        }
    }

    Events::default()
}
