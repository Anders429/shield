use crate::{
    components, constants,
    events::Events,
    system::{find_pixel_difference, movement},
    World,
};
use itertools::izip;

pub(crate) fn follow_player<const ENTITY_COUNT: usize>(world: &mut World<ENTITY_COUNT>) -> Events {
    let mut events = Events::default();

    // Get player location.
    let mut player_position = components::Position::default();
    let mut player_chunk = components::Chunk::default();
    for (entity, position, chunk) in izip!(
        world.entities.iter(),
        world.components.positions.iter(),
        world.components.chunks.iter()
    ) {
        if entity.is_player() && entity.has_position() && entity.has_chunk() {
            player_position = position.clone();
            player_chunk = chunk.clone();
            break;
        }
    }

    for (entity, position, chunk, facing_direction, moving_direction, accepts_input, retreating) in izip!(
        world.entities.iter(),
        world.components.positions.iter_mut(),
        world.components.chunks.iter_mut(),
        world.components.facing_directions.iter_mut(),
        world.components.moving_directions.iter_mut(),
        world.components.accepts_input.iter(),
        world.components.retreatings.iter()
    ) {
        if entity.has_accepts_input()
            && accepts_input.follows_player()
            && entity.has_position()
            && entity.has_chunk()
            && entity.has_facing_direction()
        {
            let mut x_delta = find_pixel_difference(
                position.x,
                chunk.x,
                player_position.x,
                player_chunk.x,
                constants::CHUNK_WIDTH,
            );
            let mut y_delta = find_pixel_difference(
                position.y,
                chunk.y,
                player_position.y,
                player_chunk.y,
                constants::CHUNK_HEIGHT,
            );

            if entity.has_retreating() {
                if *retreating > 0 {
                    x_delta = -x_delta;
                    y_delta = -y_delta;
                }
            }

            match facing_direction {
                components::Direction::Up => {
                    if y_delta > 0 {
                        events |= movement(position, chunk, *facing_direction, 1);
                        *moving_direction = *facing_direction;
                    } else {
                        if x_delta < 0 {
                            *facing_direction = components::Direction::Right;
                        } else {
                            *facing_direction = components::Direction::Left;
                        }
                    }
                }
                components::Direction::Right => {
                    if x_delta < 0 {
                        events |= movement(position, chunk, *facing_direction, 1);
                        *moving_direction = *facing_direction;
                    } else {
                        if y_delta < 0 {
                            *facing_direction = components::Direction::Down;
                        } else {
                            *facing_direction = components::Direction::Up;
                        }
                    }
                }
                components::Direction::Down => {
                    if y_delta < 0 {
                        events |= movement(position, chunk, *facing_direction, 1);
                        *moving_direction = *facing_direction;
                    } else {
                        if x_delta < 0 {
                            *facing_direction = components::Direction::Right;
                        } else {
                            *facing_direction = components::Direction::Left;
                        }
                    }
                }
                components::Direction::Left => {
                    if x_delta > 0 {
                        events |= movement(position, chunk, *facing_direction, 1);
                        *moving_direction = *facing_direction;
                    } else {
                        if y_delta < 0 {
                            *facing_direction = components::Direction::Down;
                        } else {
                            *facing_direction = components::Direction::Up;
                        }
                    }
                }
            }
        }
    }

    events
}
