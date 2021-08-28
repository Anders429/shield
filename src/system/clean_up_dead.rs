use crate::{components, events::Events, entity::Entity, generational_index::GenerationalIndex, World};
use itertools::izip;

pub(crate) fn clean_up_dead<const ENTITY_COUNT: usize>(world: &mut World<ENTITY_COUNT>) -> Events {
    for (index, (entity, generation, health_points)) in izip!(world.entities.iter_mut(), world.components.generations.iter(), world.components.health_points.iter()).enumerate() {
        if entity.has_generation() && entity.has_health_points() && health_points.current == 0 {
            if entity.is_player() {
                world.resources.game_state = components::GameState::GameOver;
                return Events::default();
            }
            unsafe {world.generational_index_allocator.deallocate_unchecked(GenerationalIndex {
                index, generation: *generation,
            })};
            *entity = Entity::default();
        }
    }

    Events::default()
}
