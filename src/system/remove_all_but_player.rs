use crate::{components, events::Events, entity::Entity, World};

pub(crate) fn remove_all_but_player<const ENTITY_COUNT: usize>(world: &mut World<ENTITY_COUNT>) -> Events {
    for entity in world.entities.iter_mut() {
        if !entity.is_player() {
            *entity = Entity::default();
        }
    }

    Events::default()
}
