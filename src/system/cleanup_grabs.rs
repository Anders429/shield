use crate::{components, entity::Entity, events::Events, generational_index::GenerationalIndex, World};
use itertools::izip;

pub(crate) fn cleanup_grabs<const ENTITY_COUNT: usize>(world: &mut World<ENTITY_COUNT>) -> Events {
    for (index, (entity, generation)) in izip!(world.entities.iter_mut(), world.components.generations.iter()).enumerate() {
        if entity.has_grab() && entity.has_generation() {
            unsafe {
                world.generational_index_allocator.deallocate_unchecked(GenerationalIndex {
                    index,
                    generation: *generation,
                });
            }
            *entity = Entity::default();
        }
    }

    Events::default()
}
