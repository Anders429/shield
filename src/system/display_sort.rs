use crate::{constants, entity::Entity, system::find_pixel_difference, World};
use std::ops::FnMut;

pub(crate) fn display_sort<F, const ENTITY_COUNT: usize>(
    world: &mut World<ENTITY_COUNT>,
    filter_map: F,
) -> Vec<usize>
where
    F: FnMut((usize, &Entity)) -> Option<usize>,
{
    // Sort (by y axis).
    let mut sorted = world
        .entities
        .iter()
        .enumerate()
        .filter(|(index, entity)| entity.has_position() && entity.has_chunk())
        .filter_map(filter_map)
        .collect::<Vec<_>>();
    sorted.sort_unstable_by(|index_a, index_b| {
        let position_a = unsafe { world.components.positions.get_unchecked(*index_a) };
        let chunk_a = unsafe { world.components.chunks.get_unchecked(*index_a) };
        let position_b = unsafe { world.components.positions.get_unchecked(*index_b) };
        let chunk_b = unsafe { world.components.chunks.get_unchecked(*index_b) };

        let y_a =
            find_pixel_difference(position_a.y, chunk_a.y, 0, 0, constants::CHUNK_HEIGHT) as u16;
        let y_b =
            find_pixel_difference(position_b.y, chunk_b.y, 0, 0, constants::CHUNK_HEIGHT) as u16;

        let start_a = y_a;
        let end_a = start_a + constants::SPRITE_HEIGHT as u16;
        let start_b = y_b;
        let end_b = start_b + constants::SPRITE_HEIGHT as u16;

        start_a.cmp(&start_b).then_with(|| end_b.cmp(&end_a))
    });

    sorted
}
