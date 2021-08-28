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

                let start_a = y_a + bounding_box_a.offset_y as u16;
                let end_a = start_a + bounding_box_a.height as u16;
                let start_b = y_b + bounding_box_b.offset_y as u16;
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
        events |= collision_held_immovable(world, *index_a, *index_b);
        events |= collision_held_immovable(world, *index_b, *index_a);
        events |= collision_movables(world, *index_a, *index_b);
        events |= collision_held_movable(world, *index_a, *index_b);
        events |= collision_held_movable(world, *index_b, *index_a);
        events |= collision_damage(world, *index_a, *index_b);
        events |= collision_damage(world, *index_b, *index_a);
        events |= collision_grab_holdable(world, *index_a, *index_b);
        events |= collision_grab_holdable(world, *index_b, *index_a);
    }

    events
}

fn collides<const ENTITY_COUNT: usize>(
    world: &mut World<ENTITY_COUNT>,
    index_a: usize,
    index_b: usize,
) -> bool {
    // Collides on x.
    let position_a = unsafe { world.components.positions.get_unchecked(index_a) };
    let chunk_a = unsafe { world.components.chunks.get_unchecked(index_a) };
    let bounding_box_a = unsafe { world.components.bounding_boxes.get_unchecked(index_a) };
    let position_b = unsafe { world.components.positions.get_unchecked(index_b) };
    let chunk_b = unsafe { world.components.chunks.get_unchecked(index_b) };
    let bounding_box_b = unsafe { world.components.bounding_boxes.get_unchecked(index_b) };

    let x_a = find_pixel_difference(position_a.x, chunk_a.x, 0, 0, constants::CHUNK_WIDTH) as u16;
    let x_b = find_pixel_difference(position_b.x, chunk_b.x, 0, 0, constants::CHUNK_WIDTH) as u16;

    let start_a = x_a + bounding_box_a.offset_x as u16;
    let end_a = start_a + bounding_box_a.width as u16;
    let start_b = x_b + bounding_box_b.offset_x as u16;
    let end_b = start_b + bounding_box_b.width as u16;

    if (start_a < end_b && end_a > start_b) || (end_b < start_a && start_b > end_a) {
        // Collides on y.
        let y_a =
            find_pixel_difference(position_a.y, chunk_a.y, 0, 0, constants::CHUNK_HEIGHT) as u16;
        let y_b =
            find_pixel_difference(position_b.y, chunk_b.y, 0, 0, constants::CHUNK_HEIGHT) as u16;

        let start_a = y_a + bounding_box_a.offset_y as u16;
        let end_a = start_a + bounding_box_a.height as u16;
        let start_b = y_b + bounding_box_b.offset_y as u16;
        let end_b = start_b + bounding_box_b.height as u16;
        if (start_a < end_b && end_a > start_b) || (end_b < start_a && start_b > end_a) {
            return true;
        }
    }
    false
}

fn collision_moving_direction_immovable<const ENTITY_COUNT: usize>(
    world: &mut World<ENTITY_COUNT>,
    index_a: usize,
    index_b: usize,
) -> Events {
    let mut events = Events::default();

    if !collides(world, index_a, index_b) {
        return events;
    }

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
        let holding_a = unsafe { world.components.holdings.get_unchecked(index_a) };

        match unsafe { world.components.moving_directions.get_unchecked(index_a) } {
            components::Direction::Up => {
                let y_a =
                    find_pixel_difference(position_a.y, chunk_a.y, 0, 0, constants::CHUNK_HEIGHT);
                let y_b =
                    find_pixel_difference(position_b.y, chunk_b.y, 0, 0, constants::CHUNK_HEIGHT);
                let adjustment_amount =
                    (y_b + bounding_box_b.offset_y as i16 + bounding_box_b.height as i16
                        - (y_a + bounding_box_a.offset_y as i16))
                        .unsigned_abs() as u8;

                events |= movement(
                    position_a,
                    chunk_a,
                    components::Direction::Down,
                    adjustment_amount,
                );

                if entity_a.has_holding() {
                    let held_entity = unsafe { world.entities.get_unchecked_mut(holding_a.index) };
                    if unsafe {
                        world
                            .generational_index_allocator
                            .is_allocated_unchecked(*holding_a)
                    } && held_entity.has_position()
                        && held_entity.has_chunk()
                    {
                        events |= movement(
                            unsafe {
                                world
                                    .components
                                    .positions
                                    .get_unchecked_mut(holding_a.index)
                            },
                            unsafe { world.components.chunks.get_unchecked_mut(holding_a.index) },
                            components::Direction::Down,
                            adjustment_amount,
                        );
                    } else {
                        unsafe { world.entities.get_unchecked_mut(index_a) }.remove_holding();
                    }
                }
            }
            components::Direction::Right => {
                let x_a =
                    find_pixel_difference(position_a.x, chunk_a.x, 0, 0, constants::CHUNK_WIDTH);
                let x_b =
                    find_pixel_difference(position_b.x, chunk_b.x, 0, 0, constants::CHUNK_WIDTH);
                let adjustment_amount =
                    (x_a + bounding_box_a.offset_x as i16 + bounding_box_a.width as i16
                        - (x_b + bounding_box_b.offset_x as i16))
                        .unsigned_abs() as u8;

                events |= movement(
                    position_a,
                    chunk_a,
                    components::Direction::Left,
                    adjustment_amount,
                );

                if entity_a.has_holding() {
                    let held_entity = unsafe { world.entities.get_unchecked_mut(holding_a.index) };
                    if unsafe {
                        world
                            .generational_index_allocator
                            .is_allocated_unchecked(*holding_a)
                    } && held_entity.has_position()
                        && held_entity.has_chunk()
                    {
                        events |= movement(
                            unsafe {
                                world
                                    .components
                                    .positions
                                    .get_unchecked_mut(holding_a.index)
                            },
                            unsafe { world.components.chunks.get_unchecked_mut(holding_a.index) },
                            components::Direction::Left,
                            adjustment_amount,
                        );
                    } else {
                        unsafe { world.entities.get_unchecked_mut(index_a) }.remove_holding();
                    }
                }
            }
            components::Direction::Down => {
                let y_a =
                    find_pixel_difference(position_a.y, chunk_a.y, 0, 0, constants::CHUNK_HEIGHT);
                let y_b =
                    find_pixel_difference(position_b.y, chunk_b.y, 0, 0, constants::CHUNK_HEIGHT);
                let adjustment_amount =
                    (y_a + bounding_box_a.offset_y as i16 + bounding_box_a.height as i16
                        - (y_b + bounding_box_b.offset_y as i16))
                        .unsigned_abs() as u8;

                events |= movement(
                    position_a,
                    chunk_a,
                    components::Direction::Up,
                    adjustment_amount,
                );

                if entity_a.has_holding() {
                    let held_entity = unsafe { world.entities.get_unchecked_mut(holding_a.index) };
                    if unsafe {
                        world
                            .generational_index_allocator
                            .is_allocated_unchecked(*holding_a)
                    } && held_entity.has_position()
                        && held_entity.has_chunk()
                    {
                        events |= movement(
                            unsafe {
                                world
                                    .components
                                    .positions
                                    .get_unchecked_mut(holding_a.index)
                            },
                            unsafe { world.components.chunks.get_unchecked_mut(holding_a.index) },
                            components::Direction::Up,
                            adjustment_amount,
                        );
                    } else {
                        unsafe { world.entities.get_unchecked_mut(index_a) }.remove_holding();
                    }
                }
            }
            components::Direction::Left => {
                let x_a =
                    find_pixel_difference(position_a.x, chunk_a.x, 0, 0, constants::CHUNK_WIDTH);
                let x_b =
                    find_pixel_difference(position_b.x, chunk_b.x, 0, 0, constants::CHUNK_WIDTH);
                let adjustment_amount =
                    (x_b + bounding_box_b.offset_x as i16 + bounding_box_b.width as i16
                        - (x_a + bounding_box_a.offset_x as i16))
                        .unsigned_abs() as u8;

                events |= movement(
                    position_a,
                    chunk_a,
                    components::Direction::Right,
                    adjustment_amount,
                );

                if entity_a.has_holding() {
                    let held_entity = unsafe { world.entities.get_unchecked_mut(holding_a.index) };
                    if unsafe {
                        world
                            .generational_index_allocator
                            .is_allocated_unchecked(*holding_a)
                    } && held_entity.has_position()
                        && held_entity.has_chunk()
                    {
                        events |= movement(
                            unsafe {
                                world
                                    .components
                                    .positions
                                    .get_unchecked_mut(holding_a.index)
                            },
                            unsafe { world.components.chunks.get_unchecked_mut(holding_a.index) },
                            components::Direction::Right,
                            adjustment_amount,
                        );
                    } else {
                        unsafe { world.entities.get_unchecked_mut(index_a) }.remove_holding();
                    }
                }
            }
        }
    }

    events
}

fn collision_held_immovable<const ENTITY_COUNT: usize>(
    world: &mut World<ENTITY_COUNT>,
    index_a: usize,
    index_b: usize,
) -> Events {
    let mut events = Events::default();

    if !collides(world, index_a, index_b) {
        return events;
    }

    let entity_a = unsafe { world.entities.get_unchecked(index_a) };
    let entity_b = unsafe { world.entities.get_unchecked(index_b) };

    let position_b = unsafe { world.components.positions.get_unchecked(index_b) }.clone();
    let chunk_b = unsafe { world.components.chunks.get_unchecked(index_b) }.clone();
    let bounding_box_b = unsafe { world.components.bounding_boxes.get_unchecked(index_b) };
    let position_a = unsafe { world.components.positions.get_unchecked_mut(index_a) };
    let chunk_a = unsafe { world.components.chunks.get_unchecked_mut(index_a) };
    let bounding_box_a = unsafe { world.components.bounding_boxes.get_unchecked(index_a) };

    if entity_a.has_held() && entity_b.is_immovable() {
        let held_by = unsafe { world.components.helds.get_unchecked(index_a) };
        let holding_entity = unsafe { world.entities.get_unchecked(held_by.index) };
        if unsafe {
            world
                .generational_index_allocator
                .is_allocated_unchecked(*held_by)
        } && holding_entity.has_position()
            && holding_entity.has_chunk()
            && holding_entity.has_moving_direction()
        {
            match unsafe {
                world
                    .components
                    .moving_directions
                    .get_unchecked(held_by.index)
            } {
                components::Direction::Up => {
                    let y_a = find_pixel_difference(
                        position_a.y,
                        chunk_a.y,
                        0,
                        0,
                        constants::CHUNK_HEIGHT,
                    );
                    let y_b = find_pixel_difference(
                        position_b.y,
                        chunk_b.y,
                        0,
                        0,
                        constants::CHUNK_HEIGHT,
                    );
                    let adjustment_amount =
                        (y_b + bounding_box_b.offset_y as i16 + bounding_box_b.height as i16
                            - (y_a + bounding_box_a.offset_y as i16))
                            .unsigned_abs() as u8;

                    events |= movement(
                        position_a,
                        chunk_a,
                        components::Direction::Down,
                        adjustment_amount,
                    );

                    events |= movement(
                        unsafe { world.components.positions.get_unchecked_mut(held_by.index) },
                        unsafe { world.components.chunks.get_unchecked_mut(held_by.index) },
                        components::Direction::Down,
                        adjustment_amount,
                    );
                }
                components::Direction::Right => {
                    let x_a = find_pixel_difference(
                        position_a.x,
                        chunk_a.x,
                        0,
                        0,
                        constants::CHUNK_WIDTH,
                    );
                    let x_b = find_pixel_difference(
                        position_b.x,
                        chunk_b.x,
                        0,
                        0,
                        constants::CHUNK_WIDTH,
                    );
                    let adjustment_amount =
                        (x_a + bounding_box_a.offset_x as i16 + bounding_box_a.width as i16
                            - (x_b + bounding_box_b.offset_x as i16))
                            .unsigned_abs() as u8;

                    events |= movement(
                        position_a,
                        chunk_a,
                        components::Direction::Left,
                        adjustment_amount,
                    );

                    events |= movement(
                        unsafe { world.components.positions.get_unchecked_mut(held_by.index) },
                        unsafe { world.components.chunks.get_unchecked_mut(held_by.index) },
                        components::Direction::Left,
                        adjustment_amount,
                    );
                }
                components::Direction::Down => {
                    let y_a = find_pixel_difference(
                        position_a.y,
                        chunk_a.y,
                        0,
                        0,
                        constants::CHUNK_HEIGHT,
                    );
                    let y_b = find_pixel_difference(
                        position_b.y,
                        chunk_b.y,
                        0,
                        0,
                        constants::CHUNK_HEIGHT,
                    );
                    let adjustment_amount =
                        (y_a + bounding_box_a.offset_y as i16 + bounding_box_a.height as i16
                            - (y_b + bounding_box_b.offset_y as i16))
                            .unsigned_abs() as u8;

                    events |= movement(
                        position_a,
                        chunk_a,
                        components::Direction::Up,
                        adjustment_amount,
                    );

                    events |= movement(
                        unsafe { world.components.positions.get_unchecked_mut(held_by.index) },
                        unsafe { world.components.chunks.get_unchecked_mut(held_by.index) },
                        components::Direction::Up,
                        adjustment_amount,
                    );
                }
                components::Direction::Left => {
                    let x_a = find_pixel_difference(
                        position_a.x,
                        chunk_a.x,
                        0,
                        0,
                        constants::CHUNK_WIDTH,
                    );
                    let x_b = find_pixel_difference(
                        position_b.x,
                        chunk_b.x,
                        0,
                        0,
                        constants::CHUNK_WIDTH,
                    );
                    let adjustment_amount =
                        (x_b + bounding_box_b.offset_x as i16 + bounding_box_b.width as i16
                            - (x_a + bounding_box_a.offset_x as i16))
                            .unsigned_abs() as u8;

                    events |= movement(
                        position_a,
                        chunk_a,
                        components::Direction::Right,
                        adjustment_amount,
                    );

                    events |= movement(
                        unsafe { world.components.positions.get_unchecked_mut(held_by.index) },
                        unsafe { world.components.chunks.get_unchecked_mut(held_by.index) },
                        components::Direction::Right,
                        adjustment_amount,
                    );
                }
            }
        } else {
            // Arbitrarily move down.
            let y_a = find_pixel_difference(position_a.y, chunk_a.y, 0, 0, constants::CHUNK_HEIGHT);
            let y_b = find_pixel_difference(position_b.y, chunk_b.y, 0, 0, constants::CHUNK_HEIGHT);
            let adjustment_amount =
                (y_b + bounding_box_b.offset_y as i16 + bounding_box_b.height as i16
                    - (y_a + bounding_box_a.offset_y as i16))
                    .unsigned_abs() as u8;

            events |= movement(
                position_a,
                chunk_a,
                components::Direction::Down,
                adjustment_amount,
            );

            // Remove the invalid entity reference.
            unsafe { world.entities.get_unchecked_mut(index_a) }.remove_held();
        }
    }

    events
}

// Only needs to be done one direction.
fn collision_movables<const ENTITY_COUNT: usize>(
    world: &mut World<ENTITY_COUNT>,
    index_a: usize,
    index_b: usize,
) -> Events {
    let mut events = Events::default();

    if !collides(world, index_a, index_b) {
        return events;
    }

    let entity_a = unsafe { world.entities.get_unchecked(index_a) }.clone();
    let entity_b = unsafe { world.entities.get_unchecked(index_b) }.clone();

    let position_a = unsafe { world.components.positions.get_unchecked(index_a) };
    let chunk_a = unsafe { world.components.chunks.get_unchecked(index_a) };
    let bounding_box_a = unsafe { world.components.bounding_boxes.get_unchecked(index_a) };
    let holding_a = unsafe { world.components.holdings.get_unchecked(index_a) };
    let position_b = unsafe { world.components.positions.get_unchecked(index_b) }.clone();
    let chunk_b = unsafe { world.components.chunks.get_unchecked(index_b) }.clone();
    let bounding_box_b = unsafe { world.components.bounding_boxes.get_unchecked(index_b) };
    let holding_b = unsafe { world.components.holdings.get_unchecked(index_b) };

    if entity_a.has_moving_direction() && entity_b.has_moving_direction() {
        match unsafe { world.components.facing_directions.get_unchecked(index_a) } {
            components::Direction::Up => {
                let y_a =
                    find_pixel_difference(position_a.y, chunk_a.y, 0, 0, constants::CHUNK_HEIGHT);
                let y_b =
                    find_pixel_difference(position_b.y, chunk_b.y, 0, 0, constants::CHUNK_HEIGHT);
                let adjustment_amount =
                    (y_b + bounding_box_b.offset_y as i16 + bounding_box_b.height as i16
                        - (y_a + bounding_box_a.offset_y as i16))
                        .unsigned_abs() as u8;
                let adjustment_amount_a = adjustment_amount / 2;
                let adjustment_amount_b = adjustment_amount - adjustment_amount_a;

                let position_a = unsafe { world.components.positions.get_unchecked_mut(index_a) };
                let chunk_a = unsafe { world.components.chunks.get_unchecked_mut(index_a) };
                events |= movement(
                    position_a,
                    chunk_a,
                    components::Direction::Down,
                    adjustment_amount_a,
                );
                let position_b = unsafe { world.components.positions.get_unchecked_mut(index_b) };
                let chunk_b = unsafe { world.components.chunks.get_unchecked_mut(index_b) };
                events |= movement(
                    position_b,
                    chunk_b,
                    components::Direction::Up,
                    adjustment_amount_b,
                );

                if entity_a.has_holding() {
                    let held_entity = unsafe { world.entities.get_unchecked_mut(holding_a.index) };
                    if unsafe {
                        world
                            .generational_index_allocator
                            .is_allocated_unchecked(*holding_a)
                    } && held_entity.has_position()
                        && held_entity.has_chunk()
                    {
                        events |= movement(
                            unsafe {
                                world
                                    .components
                                    .positions
                                    .get_unchecked_mut(holding_a.index)
                            },
                            unsafe { world.components.chunks.get_unchecked_mut(holding_a.index) },
                            components::Direction::Down,
                            adjustment_amount_a,
                        );
                    } else {
                        unsafe { world.entities.get_unchecked_mut(index_a) }.remove_holding();
                    }
                }
                if entity_b.has_holding() {
                    let held_entity = unsafe { world.entities.get_unchecked_mut(holding_b.index) };
                    if unsafe {
                        world
                            .generational_index_allocator
                            .is_allocated_unchecked(*holding_b)
                    } && held_entity.has_position()
                        && held_entity.has_chunk()
                    {
                        events |= movement(
                            unsafe {
                                world
                                    .components
                                    .positions
                                    .get_unchecked_mut(holding_b.index)
                            },
                            unsafe { world.components.chunks.get_unchecked_mut(holding_b.index) },
                            components::Direction::Up,
                            adjustment_amount_b,
                        );
                    } else {
                        unsafe { world.entities.get_unchecked_mut(index_a) }.remove_holding();
                    }
                }
            }
            components::Direction::Right => {
                let x_a =
                    find_pixel_difference(position_a.x, chunk_a.x, 0, 0, constants::CHUNK_WIDTH);
                let x_b =
                    find_pixel_difference(position_b.x, chunk_b.x, 0, 0, constants::CHUNK_WIDTH);
                let adjustment_amount =
                    (x_a + bounding_box_a.offset_x as i16 + bounding_box_a.width as i16
                        - (x_b + bounding_box_b.offset_x as i16))
                        .unsigned_abs() as u8;
                let adjustment_amount_a = adjustment_amount / 2;
                let adjustment_amount_b = adjustment_amount - adjustment_amount_a;

                let position_a = unsafe { world.components.positions.get_unchecked_mut(index_a) };
                let chunk_a = unsafe { world.components.chunks.get_unchecked_mut(index_a) };
                events |= movement(
                    position_a,
                    chunk_a,
                    components::Direction::Left,
                    adjustment_amount_a,
                );
                let position_b = unsafe { world.components.positions.get_unchecked_mut(index_b) };
                let chunk_b = unsafe { world.components.chunks.get_unchecked_mut(index_b) };
                events |= movement(
                    position_b,
                    chunk_b,
                    components::Direction::Right,
                    adjustment_amount_b,
                );

                if entity_a.has_holding() {
                    let held_entity = unsafe { world.entities.get_unchecked_mut(holding_a.index) };
                    if unsafe {
                        world
                            .generational_index_allocator
                            .is_allocated_unchecked(*holding_a)
                    } && held_entity.has_position()
                        && held_entity.has_chunk()
                    {
                        events |= movement(
                            unsafe {
                                world
                                    .components
                                    .positions
                                    .get_unchecked_mut(holding_a.index)
                            },
                            unsafe { world.components.chunks.get_unchecked_mut(holding_a.index) },
                            components::Direction::Left,
                            adjustment_amount_a,
                        );
                    } else {
                        unsafe { world.entities.get_unchecked_mut(index_a) }.remove_holding();
                    }
                }
                if entity_b.has_holding() {
                    let held_entity = unsafe { world.entities.get_unchecked_mut(holding_b.index) };
                    if unsafe {
                        world
                            .generational_index_allocator
                            .is_allocated_unchecked(*holding_b)
                    } && held_entity.has_position()
                        && held_entity.has_chunk()
                    {
                        events |= movement(
                            unsafe {
                                world
                                    .components
                                    .positions
                                    .get_unchecked_mut(holding_b.index)
                            },
                            unsafe { world.components.chunks.get_unchecked_mut(holding_b.index) },
                            components::Direction::Right,
                            adjustment_amount_b,
                        );
                    } else {
                        unsafe { world.entities.get_unchecked_mut(index_a) }.remove_holding();
                    }
                }
            }
            components::Direction::Down => {
                let y_a =
                    find_pixel_difference(position_a.y, chunk_a.y, 0, 0, constants::CHUNK_HEIGHT);
                let y_b =
                    find_pixel_difference(position_b.y, chunk_b.y, 0, 0, constants::CHUNK_HEIGHT);
                let adjustment_amount =
                    (y_a + bounding_box_a.offset_y as i16 + bounding_box_a.height as i16
                        - (y_b + bounding_box_b.offset_y as i16))
                        .unsigned_abs() as u8;
                let adjustment_amount_a = adjustment_amount / 2;
                let adjustment_amount_b = adjustment_amount - adjustment_amount_a;

                let position_a = unsafe { world.components.positions.get_unchecked_mut(index_a) };
                let chunk_a = unsafe { world.components.chunks.get_unchecked_mut(index_a) };
                events |= movement(
                    position_a,
                    chunk_a,
                    components::Direction::Up,
                    adjustment_amount_a,
                );
                let position_b = unsafe { world.components.positions.get_unchecked_mut(index_b) };
                let chunk_b = unsafe { world.components.chunks.get_unchecked_mut(index_b) };
                events |= movement(
                    position_b,
                    chunk_b,
                    components::Direction::Down,
                    adjustment_amount_b,
                );

                if entity_a.has_holding() {
                    let held_entity = unsafe { world.entities.get_unchecked_mut(holding_a.index) };
                    if unsafe {
                        world
                            .generational_index_allocator
                            .is_allocated_unchecked(*holding_a)
                    } && held_entity.has_position()
                        && held_entity.has_chunk()
                    {
                        events |= movement(
                            unsafe {
                                world
                                    .components
                                    .positions
                                    .get_unchecked_mut(holding_a.index)
                            },
                            unsafe { world.components.chunks.get_unchecked_mut(holding_a.index) },
                            components::Direction::Up,
                            adjustment_amount_a,
                        );
                    } else {
                        unsafe { world.entities.get_unchecked_mut(index_a) }.remove_holding();
                    }
                }
                if entity_b.has_holding() {
                    let held_entity = unsafe { world.entities.get_unchecked_mut(holding_b.index) };
                    if unsafe {
                        world
                            .generational_index_allocator
                            .is_allocated_unchecked(*holding_b)
                    } && held_entity.has_position()
                        && held_entity.has_chunk()
                    {
                        events |= movement(
                            unsafe {
                                world
                                    .components
                                    .positions
                                    .get_unchecked_mut(holding_b.index)
                            },
                            unsafe { world.components.chunks.get_unchecked_mut(holding_b.index) },
                            components::Direction::Down,
                            adjustment_amount_b,
                        );
                    } else {
                        unsafe { world.entities.get_unchecked_mut(index_a) }.remove_holding();
                    }
                }
            }
            components::Direction::Left => {
                let x_a =
                    find_pixel_difference(position_a.x, chunk_a.x, 0, 0, constants::CHUNK_WIDTH);
                let x_b =
                    find_pixel_difference(position_b.x, chunk_b.x, 0, 0, constants::CHUNK_WIDTH);
                let adjustment_amount =
                    (x_b + bounding_box_b.offset_x as i16 + bounding_box_b.width as i16
                        - (x_a + bounding_box_a.offset_x as i16))
                        .unsigned_abs() as u8;
                let adjustment_amount_a = adjustment_amount / 2;
                let adjustment_amount_b = adjustment_amount - adjustment_amount_a;

                let position_a = unsafe { world.components.positions.get_unchecked_mut(index_a) };
                let chunk_a = unsafe { world.components.chunks.get_unchecked_mut(index_a) };
                events |= movement(
                    position_a,
                    chunk_a,
                    components::Direction::Right,
                    adjustment_amount_a,
                );
                let position_b = unsafe { world.components.positions.get_unchecked_mut(index_b) };
                let chunk_b = unsafe { world.components.chunks.get_unchecked_mut(index_b) };
                events |= movement(
                    position_b,
                    chunk_b,
                    components::Direction::Left,
                    adjustment_amount_b,
                );

                if entity_a.has_holding() {
                    let held_entity = unsafe { world.entities.get_unchecked_mut(holding_a.index) };
                    if unsafe {
                        world
                            .generational_index_allocator
                            .is_allocated_unchecked(*holding_a)
                    } && held_entity.has_position()
                        && held_entity.has_chunk()
                    {
                        events |= movement(
                            unsafe {
                                world
                                    .components
                                    .positions
                                    .get_unchecked_mut(holding_a.index)
                            },
                            unsafe { world.components.chunks.get_unchecked_mut(holding_a.index) },
                            components::Direction::Right,
                            adjustment_amount_a,
                        );
                    } else {
                        unsafe { world.entities.get_unchecked_mut(index_a) }.remove_holding();
                    }
                }
                if entity_b.has_holding() {
                    let held_entity = unsafe { world.entities.get_unchecked_mut(holding_b.index) };
                    if unsafe {
                        world
                            .generational_index_allocator
                            .is_allocated_unchecked(*holding_b)
                    } && held_entity.has_position()
                        && held_entity.has_chunk()
                    {
                        events |= movement(
                            unsafe {
                                world
                                    .components
                                    .positions
                                    .get_unchecked_mut(holding_b.index)
                            },
                            unsafe { world.components.chunks.get_unchecked_mut(holding_b.index) },
                            components::Direction::Left,
                            adjustment_amount_b,
                        );
                    } else {
                        unsafe { world.entities.get_unchecked_mut(index_a) }.remove_holding();
                    }
                }
            }
        }
    }

    events
}

fn collision_held_movable<const ENTITY_COUNT: usize>(
    world: &mut World<ENTITY_COUNT>,
    index_a: usize,
    index_b: usize,
) -> Events {
    let mut events = Events::default();

    if !collides(world, index_a, index_b) {
        return events;
    }

    let entity_a = unsafe { world.entities.get_unchecked(index_a) };
    let entity_b = unsafe { world.entities.get_unchecked(index_b) };

    let position_b = unsafe { world.components.positions.get_unchecked(index_b) };
    let chunk_b = unsafe { world.components.chunks.get_unchecked(index_b) };
    let bounding_box_b = unsafe { world.components.bounding_boxes.get_unchecked(index_b) };
    let position_a = unsafe { world.components.positions.get_unchecked(index_a) };
    let chunk_a = unsafe { world.components.chunks.get_unchecked(index_a) };
    let bounding_box_a = unsafe { world.components.bounding_boxes.get_unchecked(index_a) };

    if entity_a.has_held() && entity_b.has_moving_direction() {
        let held_by = unsafe { world.components.helds.get_unchecked(index_a) };
        let holding_entity = unsafe { world.entities.get_unchecked(held_by.index) };
        if unsafe {
            world
                .generational_index_allocator
                .is_allocated_unchecked(*held_by)
        } && holding_entity.has_position()
            && holding_entity.has_chunk()
            && holding_entity.has_moving_direction()
        {
            match unsafe {
                world
                    .components
                    .facing_directions
                    .get_unchecked(held_by.index)
            } {
                components::Direction::Up => {
                    let y_a = find_pixel_difference(
                        position_a.y,
                        chunk_a.y,
                        0,
                        0,
                        constants::CHUNK_HEIGHT,
                    );
                    let y_b = find_pixel_difference(
                        position_b.y,
                        chunk_b.y,
                        0,
                        0,
                        constants::CHUNK_HEIGHT,
                    );
                    let adjustment_amount =
                        (y_b + bounding_box_b.offset_y as i16 + bounding_box_b.height as i16
                            - (y_a + bounding_box_a.offset_y as i16))
                            .unsigned_abs() as u8;
                    let adjustment_amount_a = adjustment_amount / 2;
                    let adjustment_amount_b = adjustment_amount - adjustment_amount_a;

                    let position_a = unsafe { world.components.positions.get_unchecked_mut(index_a) };
                    let chunk_a = unsafe { world.components.chunks.get_unchecked_mut(index_a) };
                    events |= movement(
                        position_a,
                        chunk_a,
                        components::Direction::Down,
                        adjustment_amount_a,
                    );
                    events |= movement(
                        unsafe { world.components.positions.get_unchecked_mut(held_by.index) },
                        unsafe { world.components.chunks.get_unchecked_mut(held_by.index) },
                        components::Direction::Down,
                        adjustment_amount_a,
                    );
                    let position_b = unsafe { world.components.positions.get_unchecked_mut(index_b) };
                    let chunk_b = unsafe { world.components.chunks.get_unchecked_mut(index_b) };
                    events |= movement(
                        position_b,
                        chunk_b,
                        components::Direction::Up,
                        adjustment_amount_b,
                    );
                }
                components::Direction::Right => {
                    let x_a = find_pixel_difference(
                        position_a.x,
                        chunk_a.x,
                        0,
                        0,
                        constants::CHUNK_WIDTH,
                    );
                    let x_b = find_pixel_difference(
                        position_b.x,
                        chunk_b.x,
                        0,
                        0,
                        constants::CHUNK_WIDTH,
                    );
                    let adjustment_amount =
                        (x_a + bounding_box_a.offset_x as i16 + bounding_box_a.width as i16
                            - (x_b + bounding_box_b.offset_x as i16))
                            .unsigned_abs() as u8;
                            let adjustment_amount_a = adjustment_amount / 2;
                            let adjustment_amount_b = adjustment_amount - adjustment_amount_a;
        
                            let position_a = unsafe { world.components.positions.get_unchecked_mut(index_a) };
                            let chunk_a = unsafe { world.components.chunks.get_unchecked_mut(index_a) };
                            events |= movement(
                                position_a,
                                chunk_a,
                                components::Direction::Left,
                                adjustment_amount_a,
                            );
                            events |= movement(
                                unsafe { world.components.positions.get_unchecked_mut(held_by.index) },
                                unsafe { world.components.chunks.get_unchecked_mut(held_by.index) },
                                components::Direction::Left,
                                adjustment_amount_a,
                            );
                            let position_b = unsafe { world.components.positions.get_unchecked_mut(index_b) };
                            let chunk_b = unsafe { world.components.chunks.get_unchecked_mut(index_b) };
                            events |= movement(
                                position_b,
                                chunk_b,
                                components::Direction::Right,
                                adjustment_amount_b,
                            );
                }
                components::Direction::Down => {
                    let y_a = find_pixel_difference(
                        position_a.y,
                        chunk_a.y,
                        0,
                        0,
                        constants::CHUNK_HEIGHT,
                    );
                    let y_b = find_pixel_difference(
                        position_b.y,
                        chunk_b.y,
                        0,
                        0,
                        constants::CHUNK_HEIGHT,
                    );
                    let adjustment_amount =
                        (y_a + bounding_box_a.offset_y as i16 + bounding_box_a.height as i16
                            - (y_b + bounding_box_b.offset_y as i16))
                            .unsigned_abs() as u8;
                            let adjustment_amount_a = adjustment_amount / 2;
                            let adjustment_amount_b = adjustment_amount - adjustment_amount_a;
        
                            let position_a = unsafe { world.components.positions.get_unchecked_mut(index_a) };
                            let chunk_a = unsafe { world.components.chunks.get_unchecked_mut(index_a) };
                            events |= movement(
                                position_a,
                                chunk_a,
                                components::Direction::Up,
                                adjustment_amount_a,
                            );
                            events |= movement(
                                unsafe { world.components.positions.get_unchecked_mut(held_by.index) },
                                unsafe { world.components.chunks.get_unchecked_mut(held_by.index) },
                                components::Direction::Up,
                                adjustment_amount_a,
                            );
                            let position_b = unsafe { world.components.positions.get_unchecked_mut(index_b) };
                            let chunk_b = unsafe { world.components.chunks.get_unchecked_mut(index_b) };
                            events |= movement(
                                position_b,
                                chunk_b,
                                components::Direction::Down,
                                adjustment_amount_b,
                            );
                }
                components::Direction::Left => {
                    let x_a = find_pixel_difference(
                        position_a.x,
                        chunk_a.x,
                        0,
                        0,
                        constants::CHUNK_WIDTH,
                    );
                    let x_b = find_pixel_difference(
                        position_b.x,
                        chunk_b.x,
                        0,
                        0,
                        constants::CHUNK_WIDTH,
                    );
                    let adjustment_amount =
                        (x_b + bounding_box_b.offset_x as i16 + bounding_box_b.width as i16
                            - (x_a + bounding_box_a.offset_x as i16))
                            .unsigned_abs() as u8;
                            let adjustment_amount_a = adjustment_amount / 2;
                            let adjustment_amount_b = adjustment_amount - adjustment_amount_a;
        
                            let position_a = unsafe { world.components.positions.get_unchecked_mut(index_a) };
                            let chunk_a = unsafe { world.components.chunks.get_unchecked_mut(index_a) };
                            events |= movement(
                                position_a,
                                chunk_a,
                                components::Direction::Right,
                                adjustment_amount_a,
                            );
                            events |= movement(
                                unsafe { world.components.positions.get_unchecked_mut(held_by.index) },
                                unsafe { world.components.chunks.get_unchecked_mut(held_by.index) },
                                components::Direction::Right,
                                adjustment_amount_a,
                            );
                            let position_b = unsafe { world.components.positions.get_unchecked_mut(index_b) };
                            let chunk_b = unsafe { world.components.chunks.get_unchecked_mut(index_b) };
                            events |= movement(
                                position_b,
                                chunk_b,
                                components::Direction::Left,
                                adjustment_amount_b,
                            );
                }
            }
        } else {
            // Arbitrarily move down.
            let y_a = find_pixel_difference(position_a.y, chunk_a.y, 0, 0, constants::CHUNK_HEIGHT);
            let y_b = find_pixel_difference(position_b.y, chunk_b.y, 0, 0, constants::CHUNK_HEIGHT);
            let adjustment_amount =
                (y_b + bounding_box_b.offset_y as i16 + bounding_box_b.height as i16
                    - (y_a + bounding_box_a.offset_y as i16))
                    .unsigned_abs() as u8;

            let position_a = unsafe { world.components.positions.get_unchecked_mut(index_a) };
            let chunk_a = unsafe { world.components.chunks.get_unchecked_mut(index_a) };
            events |= movement(
                position_a,
                chunk_a,
                components::Direction::Down,
                adjustment_amount,
            );

            // Remove the invalid entity reference.
            unsafe { world.entities.get_unchecked_mut(index_a) }.remove_held();
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
        if entity_b.has_holding() {
            let holding = unsafe { world.components.holdings.get_unchecked(index_b) };
            if unsafe {
                world
                    .generational_index_allocator
                    .is_allocated_unchecked(*holding)
            } {
                if holding.index == index_a {
                    return Events::default();
                }
            } else {
                // Should remove it here, but that's a headache. TODO.
            }
        }
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
        if entity_a.has_accepts_input()
            && unsafe { world.components.accepts_input.get_unchecked(index_a) }.follows_player()
        {
            *unsafe { world.entities.get_unchecked_mut(index_a) } |= Entity::RETREATING;
            *unsafe { world.components.retreatings.get_unchecked_mut(index_a) } = 240;
        }
    }

    Events::default()
}

fn collision_grab_holdable<const ENTITY_COUNT: usize>(
    world: &mut World<ENTITY_COUNT>,
    index_a: usize,
    index_b: usize,
) -> Events {
    // A grabbing B.
    let mut events = Events::default();

    if !collides(world, index_a, index_b) {
        return events;
    }

    let entity_a = unsafe { world.entities.get_unchecked(index_a) };
    let entity_b = unsafe { world.entities.get_unchecked(index_b) };

    if entity_a.has_grab()
        && entity_b.has_holdable()
        && entity_b.has_generation()
        && !entity_b.has_held()
    {
        let grabbing = unsafe { world.components.grabs.get_unchecked(index_a) };
        let generation_b = unsafe { world.components.generations.get_unchecked(index_b) };
        if unsafe {
            world
                .generational_index_allocator
                .is_allocated_unchecked(*grabbing)
        } {
            let generation_grabbing =
                unsafe { world.components.generations.get_unchecked(grabbing.index) };
            let grabbing_entity = unsafe { world.entities.get_unchecked_mut(grabbing.index) };
            if !grabbing_entity.has_holding()
                && grabbing_entity.has_position()
                && grabbing_entity.has_chunk()
                && grabbing_entity.has_facing_direction()
                && grabbing_entity.has_bounding_box()
            {
                *grabbing_entity |= Entity::holding();
                *unsafe { world.components.holdings.get_unchecked_mut(grabbing.index) } =
                    components::EntityReference {
                        index: index_b,
                        generation: *generation_b,
                    };

                *unsafe { world.entities.get_unchecked_mut(index_b) } |= Entity::held()
                    | Entity::position()
                    | Entity::chunk()
                    | Entity::facing_direction();
                *unsafe { world.components.holdings.get_unchecked_mut(index_b) } =
                    components::EntityReference {
                        index: grabbing.index,
                        generation: *generation_grabbing,
                    };
                let held_facing_direction = unsafe {
                    world
                        .components
                        .facing_directions
                        .get_unchecked(grabbing.index)
                }
                .clone();
                let mut held_position =
                    unsafe { world.components.positions.get_unchecked(grabbing.index) }.clone();
                let mut held_chunk =
                    unsafe { world.components.chunks.get_unchecked(grabbing.index) }.clone();
                let holder_bounding_box = unsafe {
                    world
                        .components
                        .bounding_boxes
                        .get_unchecked(grabbing.index)
                };
                events |= movement(
                    &mut held_position,
                    &mut held_chunk,
                    components::Direction::Right,
                    holder_bounding_box.offset_x,
                );
                events |= movement(
                    &mut held_position,
                    &mut held_chunk,
                    components::Direction::Down,
                    holder_bounding_box.offset_y - 2,
                );
                events |= movement(
                    &mut held_position,
                    &mut held_chunk,
                    held_facing_direction,
                    4,
                );
                *unsafe {
                    world
                        .components
                        .facing_directions
                        .get_unchecked_mut(index_b)
                } = held_facing_direction;
                *unsafe { world.components.positions.get_unchecked_mut(index_b) } = held_position;
                *unsafe { world.components.chunks.get_unchecked_mut(index_b) } = held_chunk;
            }
        }
    }

    events
}
