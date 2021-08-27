use crate::{components, events::Events, World};
use array_init::array_init;

pub(crate) fn collisions<const ENTITY_COUNT: usize>(world: &mut World<ENTITY_COUNT>) -> Events {
    // Sort (by x axis).
    let mut sorted = world.entities.iter().enumerate().filter_map(|(index, entity)| {
        if entity.has_position() && entity.has_bounding_box() {
            Some(index)
        } else {
            None
        }
    }).collect::<Vec<_>>();
    sorted.sort_unstable_by(|index_a, index_b| {
        let position_a = unsafe {world.components.positions.get_unchecked(*index_a)};
        let bounding_box_a = unsafe {world.components.bounding_boxes.get_unchecked(*index_a)};
        let position_b = unsafe {world.components.positions.get_unchecked(*index_b)};
        let bounding_box_b = unsafe {world.components.bounding_boxes.get_unchecked(*index_b)};

        let start_a = position_a.x + bounding_box_a.offset_x as u16;
        let end_a = start_a + bounding_box_a.width as u16;
        let start_b = position_b.x + bounding_box_b.offset_x as u16;
        let end_b = start_b + bounding_box_b.width as u16;

        start_a.cmp(&start_b).then_with(|| end_b.cmp(&end_a))
    });

    // Sweep.
    let mut collisions = Vec::new();
    for (i, index_a) in sorted.iter().enumerate() {
        for index_b in sorted[i+1..].iter() {
            // Collides on x.
            let position_a = unsafe {world.components.positions.get_unchecked(*index_a)};
            let bounding_box_a = unsafe {world.components.bounding_boxes.get_unchecked(*index_a)};
            let position_b = unsafe {world.components.positions.get_unchecked(*index_b)};
            let bounding_box_b = unsafe {world.components.bounding_boxes.get_unchecked(*index_b)};

            let start_a = position_a.x + bounding_box_a.offset_x as u16;
            let end_a = start_a + bounding_box_a.width as u16;
            let start_b = position_b.x + bounding_box_b.offset_x as u16;
            let end_b = start_b + bounding_box_b.width as u16;

            if (start_a < end_b && end_a > start_b) || (end_b < start_a && start_b > end_a) {
                // Collides on y.
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
    // blah blah..

    Events::default()
}