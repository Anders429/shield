use crate::{
    components, constants,
    entity::Entity,
    events::{ChunkChange, Events, Input},
    system::{find_pixel_difference, movement, rotate},
    World,
};
use itertools::izip;

pub(crate) fn player_input<const ENTITY_COUNT: usize>(
    world: &mut World<ENTITY_COUNT>,
    input: Input,
) -> Events {
    let mut events = Events::default();

    for (
        entity,
        position,
        chunk,
        accepts_input,
        speed,
        facing_direction,
        walking_timer,
        walking_animation_state,
        moving_direction,
    ) in izip!(
        world.entities.iter_mut(),
        world.components.positions.iter_mut(),
        world.components.chunks.iter_mut(),
        world.components.accepts_input.iter(),
        world.components.speeds.iter(),
        world.components.facing_directions.iter_mut(),
        world.components.walking_timers.iter_mut(),
        world.components.walking_animation_states.iter_mut(),
        world.components.moving_directions.iter_mut(),
    ) {
        if entity.has_position()
            && entity.has_chunk()
            && entity.has_accepts_input()
            && entity.has_speed()
            && accepts_input.from_player()
        {
            if input.has_up() {
                events |= movement(position, chunk, components::Direction::Up, *speed);
                *entity |= Entity::walking() | Entity::moving_direction();
                *moving_direction = components::Direction::Up;
            } else if input.has_right() {
                events |= movement(position, chunk, components::Direction::Right, *speed);
                *entity |= Entity::walking() | Entity::moving_direction();
                *moving_direction = components::Direction::Right;
            } else if input.has_down() {
                events |= movement(position, chunk, components::Direction::Down, *speed);
                *entity |= Entity::walking() | Entity::moving_direction();
                *moving_direction = components::Direction::Down;
            } else if input.has_left() {
                events |= movement(position, chunk, components::Direction::Left, *speed);
                *entity |= Entity::walking() | Entity::moving_direction();
                *moving_direction = components::Direction::Left;
            }

            if entity.is_player() {
                events |= scroll_screen(
                    *position,
                    *chunk,
                    &mut world.resources.position,
                    &mut world.resources.chunk,
                );
            }
        }

        if input
            .intersection(Input::UP | Input::RIGHT | Input::DOWN | Input::LEFT)
            .is_empty()
            && entity.has_accepts_input()
            && accepts_input.from_player()
        {
            entity.remove_moving_direction();
            if entity.is_walking()
                && entity.has_walking_timer()
                && entity.has_walking_animation_state()
            {
                entity.remove_walking();
                *walking_timer = 0;
                *walking_animation_state = components::WalkingAnimationState::StandingA;
            }
        }

        if !input.has_r()
            && entity.has_facing_direction()
            && entity.has_accepts_input()
            && accepts_input.from_player()
        {
            // Already facing that direction?
            let rotation_needed = !match facing_direction {
                components::Direction::Up => input.has_up(),
                components::Direction::Right => input.has_right(),
                components::Direction::Down => input.has_down(),
                components::Direction::Left => input.has_left(),
            };
            if rotation_needed {
                if input.has_up() {
                    events |= rotate(facing_direction, components::Direction::Up);
                } else if input.has_right() {
                    events |= rotate(facing_direction, components::Direction::Right);
                } else if input.has_down() {
                    events |= rotate(facing_direction, components::Direction::Down);
                } else if input.has_left() {
                    events |= rotate(facing_direction, components::Direction::Left);
                }
            }
        }
    }

    events
}

fn scroll_screen(
    player_position: components::Position,
    player_chunk: components::Chunk,
    screen_position: &mut components::Position,
    screen_chunk: &mut components::Chunk,
) -> Events {
    let mut events = Events::default();

    let x_pixel_difference = find_pixel_difference(
        player_position.x,
        player_chunk.x,
        screen_position.x,
        screen_chunk.x,
        constants::CHUNK_WIDTH,
    );
    let y_pixel_difference = find_pixel_difference(
        player_position.y,
        player_chunk.y,
        screen_position.y,
        screen_chunk.y,
        constants::CHUNK_HEIGHT,
    );

    // Move x
    let prev_x_chunk = screen_chunk.x;
    if x_pixel_difference.unsigned_abs() > constants::SCREEN_CENTER_HALF_WIDTH {
        let move_amount = x_pixel_difference.unsigned_abs() - constants::SCREEN_CENTER_HALF_WIDTH;
        if x_pixel_difference.signum() == 1 {
            movement(
                screen_position,
                screen_chunk,
                components::Direction::Right,
                move_amount as u8,
            );
        } else {
            movement(
                screen_position,
                screen_chunk,
                components::Direction::Left,
                move_amount as u8,
            );
        }
    }
    let x_chunk_change = screen_chunk.x as i8 - prev_x_chunk as i8;
    if x_chunk_change != 0 {
        events |= Events::screen_chunk_change(ChunkChange {
            x: x_chunk_change,
            y: 0,
        });
    }

    // Move y
    let prev_y_chunk = screen_chunk.y;
    if y_pixel_difference.unsigned_abs() > constants::SCREEN_CENTER_HALF_HEIGHT {
        let move_amount = y_pixel_difference.unsigned_abs() - constants::SCREEN_CENTER_HALF_HEIGHT;
        if y_pixel_difference.signum() == 1 {
            movement(
                screen_position,
                screen_chunk,
                components::Direction::Down,
                move_amount as u8,
            );
        } else {
            movement(
                screen_position,
                screen_chunk,
                components::Direction::Up,
                move_amount as u8,
            );
        }
    }
    let y_chunk_change = screen_chunk.y as i8 - prev_y_chunk as i8;
    if y_chunk_change != 0 {
        events |= Events::screen_chunk_change(ChunkChange {
            x: 0,
            y: y_chunk_change,
        });
    }

    events
}
