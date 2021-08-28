use crate::{components, constants, events::Events};
use std::cmp;

pub(crate) fn movement(
    position: &mut components::Position,
    chunk: &mut components::Chunk,
    direction: components::Direction,
    amount: u8,
) -> Events {
    match direction {
        components::Direction::Up => {
            let position_delta = cmp::min(position.y, amount as u16);
            let spillover = amount as u16 - position_delta;
            position.y -= position_delta;
            if spillover > 0 && chunk.y > 0 {
                chunk.y -= 1;
                position.y = constants::CHUNK_HEIGHT - spillover;
            }
        }
        components::Direction::Right => {
            let position_delta = cmp::min(constants::CHUNK_WIDTH - position.x - 1, amount as u16);
            let spillover = amount as u16 - position_delta;
            position.x += position_delta;
            if spillover > 0 && chunk.x < constants::WORLD_WIDTH - 1 {
                chunk.x += 1;
                position.x = spillover - 1;
            }
        }
        components::Direction::Down => {
            let position_delta = cmp::min(constants::CHUNK_HEIGHT - position.y - 1, amount as u16);
            let spillover = amount as u16 - position_delta;
            position.y += position_delta;
            if spillover > 0 && chunk.y < constants::WORLD_HEIGHT - 1 {
                chunk.y += 1;
                position.y = spillover - 1;
            }
        }
        components::Direction::Left => {
            let position_delta = cmp::min(position.x, amount as u16);
            let spillover = amount as u16 - position_delta;
            position.x -= position_delta;
            if spillover > 0 && chunk.x > 0 {
                chunk.x -= 1;
                position.x = constants::CHUNK_WIDTH - spillover;
            }
        }
    }

    Events::default()
}
