use crate::{components, constants, events::Events};

pub(crate) fn movement(
    position: &mut components::Position,
    chunk: &mut components::Chunk,
    direction: components::Direction,
    speed: components::Speed,
) -> Events {
    match direction {
        components::Direction::Up => {
            if position.y == 0 {
                if chunk.y != 0 {
                    position.y = constants::CHUNK_HEIGHT - 1;
                    chunk.y -= 1;
                }
            } else {
                position.y -= 1;
            }
        }
        components::Direction::Right => {
            if position.x == constants::CHUNK_WIDTH - 1 {
                if chunk.x != constants::WORLD_WIDTH - 1 {
                    position.x = 0;
                    chunk.x += 1;
                }
            } else {
                position.x += 1;
            }
        }
        components::Direction::Down => {
            if position.y == constants::CHUNK_HEIGHT - 1 {
                if chunk.y != constants::WORLD_HEIGHT - 1 {
                    position.y = 0;
                    chunk.y += 1;
                }
            } else {
                position.y += 1;
            }
        }
        components::Direction::Left => {
            if position.x == 0 {
                if chunk.x != 0 {
                    position.x = constants::CHUNK_WIDTH - 1;
                    chunk.x -= 1;
                }
            } else {
                position.x -= 1;
            }
        }
    }

    Events::default()
}
