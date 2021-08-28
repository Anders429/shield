use crate::{
    components, constants,
    entity::Entity,
    events::Events,
    system::{find_pixel_difference, movement},
    World,
};
use array_init::array_init;

pub(crate) fn collisions<const ENTITY_COUNT: usize>(world: &mut World<ENTITY_COUNT>) -> Events {
    let mut events = Events::default();

    // Sort (by x axis).
    let mut sorted = world
        .entities
        .iter()
        .enumerate()
        .filter_map(|(index, entity)| {
            if entity.has_position() && entity.has_chunk() && entity.has_bounding_box() {
                Some(index)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    sorted.sort_unstable_by(|index_a, index_b| {
        let position_a = unsafe { world.components.positions.get_unchecked(*index_a) };
        let chunk_a = unsafe { world.components.chunks.get_unchecked(*index_a) };
        let bounding_box_a = unsafe { world.components.bounding_boxes.get_unchecked(*index_a) };
        let position_b = unsafe { world.components.positions.get_unchecked(*index_b) };
        let chunk_b = unsafe { world.components.chunks.get_unchecked(*index_b) };
        let bounding_box_b = unsafe { world.components.bounding_boxes.get_unchecked(*index_b) };

        let x_a =
            find_pixel_difference(position_a.x, chunk_a.x, 0, 0, constants::CHUNK_WIDTH) as u16;
        let x_b =
            find_pixel_difference(position_b.x, chunk_b.x, 0, 0, constants::CHUNK_WIDTH) as u16;

        let start_a = x_a + bounding_box_a.offset_x as u16;
        let end_a = start_a + bounding_box_a.width as u16;
        let start_b = x_b + bounding_box_b.offset_x as u16;
        let end_b = start_b + bounding_box_b.width as u16;

        start_a.cmp(&start_b).then_with(|| end_b.cmp(&end_a))
    });

    // Sweep.
    let mut collisions = Vec::new();
    for (i, index_a) in sorted.iter().enumerate() {
        for index_b in sorted[i + 1..].iter() {
            // Collides on x.
            let position_a = unsafe { world.components.positions.get_unchecked(*index_a) };
            let chunk_a = unsafe { world.components.chunks.get_unchecked(*index_a) };
            let bounding_box_a = unsafe { world.components.bounding_boxes.get_unchecked(*index_a) };
            let position_b = unsafe { world.components.positions.get_unchecked(*index_b) };
            let chunk_b = unsafe { world.components.chunks.get_unchecked(*index_b) };
            let bounding_box_b = unsafe { world.components.bounding_boxes.get_unchecked(*index_b) };

            let x_a =
                find_pixel_difference(position_a.x, chunk_a.x, 0, 0, constants::CHUNK_WIDTH) as u16;
            let x_b =
                find_pixel_difference(position_b.x, chunk_b.x, 0, 0, constants::CHUNK_WIDTH) as u16;

            let start_a = x_a + bounding_box_a.offset_x as u16;
            let end_a = start_a + bounding_box_a.width as u16;
            let start_b = x_b + bounding_box_b.offset_x as u16;
            let end_b = start_b + bounding_box_b.width as u16;

            if (start_a < end_b && end_a > start_b) || (end_b < start_a && start_b > end_a) {
                // Collides on y.
                let y_a =
                    find_pixel_difference(position_a.y, chunk_a.y, 0, 0, constants::CHUNK_HEIGHT)
                        as u16;
                let y_b =
                    find_pixel_difference(position_b.y, chunk_b.y, 0, 0, constants::CHUNK_HEIGHT)
                        as u16;

                let start_a = position_a.y + bounding_box_a.offset_y as u16;
                let end_a = start_a + bounding_box_a.height as u16;
                let start_b = position_b.y + bounding_box_b.offset_y as u16;
                let end_b = start_b + bounding_box_b.height as u16;
                if (start_a < end_b && end_a > start_b) || (end_b < start_a && start_b > end_a) {
                    collisions.push((index_a, index_b));
                }
            } else {
                // No more x-axis collisions.
                break;
            }
        }
    }

    // Handle collisions.
    for (index_a, index_b) in collisions {
        // Movement adjustments.
        events |= collision_moving_direction_immovable(world, *index_a, *index_b);
        events |= collision_moving_direction_immovable(world, *index_b, *index_a);
        events |= collision_damage(world, *index_a, *index_b);
        events |= collision_damage(world, *index_b, *index_a);
    }

    events
}

fn collision_moving_direction_immovable<const ENTITY_COUNT: usize>(
    world: &mut World<ENTITY_COUNT>,
    index_a: usize,
    index_b: usize,
) -> Events {
    let mut events = Events::default();

    let entity_a = unsafe { world.entities.get_unchecked(index_a) };
    let entity_b = unsafe { world.entities.get_unchecked(index_b) };

    if entity_a.has_moving_direction() && !entity_a.is_immovable() && entity_b.is_immovable() {
        // Copying to allow mutation at same time.
        let position_b = unsafe { world.components.positions.get_unchecked(index_b) }.clone();
        let chunk_b = unsafe { world.components.chunks.get_unchecked(index_b) }.clone();
        let bounding_box_b = unsafe { world.components.bounding_boxes.get_unchecked(index_b) };

        let position_a = unsafe { world.components.positions.get_unchecked_mut(index_a) };
        let chunk_a = unsafe { world.components.chunks.get_unchecked_mut(index_a) };
        let bounding_box_a = unsafe { world.components.bounding_boxes.get_unchecked(index_a) };

        match unsafe { world.components.moving_directions.get_unchecked(index_a) } {
            components::Direction::Up => {
                let y_a =
                    find_pixel_difference(position_a.y, chunk_a.y, 0, 0, constants::CHUNK_HEIGHT);
                let y_b =
                    find_pixel_difference(position_b.y, chunk_b.y, 0, 0, constants::CHUNK_HEIGHT);

                events |= movement(
                    position_a,
                    chunk_a,
                    components::Direction::Down,
                    (y_b + bounding_box_b.offset_y as i16 + bounding_box_b.height as i16
                        - (y_a + bounding_box_a.offset_y as i16))
                        .unsigned_abs() as u8,
                );
            }
            components::Direction::Right => {
                let x_a =
                    find_pixel_difference(position_a.x, chunk_a.x, 0, 0, constants::CHUNK_WIDTH);
                let x_b =
                    find_pixel_difference(position_b.x, chunk_b.x, 0, 0, constants::CHUNK_WIDTH);

                events |= movement(
                    position_a,
                    chunk_a,
                    components::Direction::Left,
                    (x_a + bounding_box_a.offset_x as i16 + bounding_box_a.width as i16
                        - (x_b + bounding_box_b.offset_x as i16))
                        .unsigned_abs() as u8,
                );
            }
            components::Direction::Down => {
                let y_a =
                    find_pixel_difference(position_a.y, chunk_a.y, 0, 0, constants::CHUNK_HEIGHT);
                let y_b =
                    find_pixel_difference(position_b.y, chunk_b.y, 0, 0, constants::CHUNK_HEIGHT);

                events |= movement(
                    position_a,
                    chunk_a,
                    components::Direction::Up,
                    (y_a + bounding_box_a.offset_y as i16 + bounding_box_a.height as i16
                        - (y_b + bounding_box_b.offset_y as i16))
                        .unsigned_abs() as u8,
                );
            }
            components::Direction::Left => {
                let x_a =
                    find_pixel_difference(position_a.x, chunk_a.x, 0, 0, constants::CHUNK_WIDTH);
                let x_b =
                    find_pixel_difference(position_b.x, chunk_b.x, 0, 0, constants::CHUNK_WIDTH);

                events |= movement(
                    position_a,
                    chunk_a,
                    components::Direction::Right,
                    (x_b + bounding_box_b.offset_x as i16 + bounding_box_b.width as i16
                        - (x_a + bounding_box_a.offset_x as i16))
                        .unsigned_abs() as u8,
                );
            }
        }
    }

    events
}

fn collision_damage<const ENTITY_COUNT: usize>(
    world: &mut World<ENTITY_COUNT>,
    index_a: usize,
    index_b: usize,
) -> Events {
    // A damages B.
    let entity_a = unsafe { world.entities.get_unchecked(index_a) }.clone();
    let entity_b = unsafe { world.entities.get_unchecked_mut(index_b) };

    if entity_a.has_damage()
        && entity_b.has_health_points()
        && !entity_b.has_damage_invulnerability_timer()
    {
        unsafe {
            let health_points = world.components.health_points.get_unchecked_mut(index_b);
            health_points.current = health_points
                .current
                .saturating_sub(*world.components.damages.get_unchecked(index_a));
            *entity_b |= Entity::damage_invulnerability_timer();
            *world
                .components
                .damage_invulnerability_timers
                .get_unchecked_mut(index_b) = 60;
        }
    }

    Events::default()
}
