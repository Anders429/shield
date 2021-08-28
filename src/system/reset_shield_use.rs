use crate::{components, events::Events, system::movement, World};
use itertools::izip;

pub(crate) fn reset_shield_use<const ENTITY_COUNT: usize>(
    world: &mut World<ENTITY_COUNT>,
) -> Events {
    let mut events = Events::default();

    for (entity, position, chunk, facing_direction, usable) in izip!(
        world.entities.iter_mut(),
        world.components.positions.iter_mut(),
        world.components.chunks.iter_mut(),
        world.components.facing_directions.iter(),
        world.components.usables.iter(),
    ) {
        if entity.has_position()
            && entity.has_chunk()
            && entity.has_facing_direction()
            && entity.has_damage()
            && entity.has_usable()
            && matches!(usable, components::Usable::Shield)
        {
            entity.remove_damage();
            events |= movement(
                position,
                chunk,
                match facing_direction {
                    components::Direction::Up => components::Direction::Down,
                    components::Direction::Right => components::Direction::Left,
                    components::Direction::Down => components::Direction::Up,
                    components::Direction::Left => components::Direction::Right,
                },
                1,
            );
        }
    }

    events
}
