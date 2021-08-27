use crate::{components, events::Events};

pub(crate) fn rotate(
    direction: &mut components::Direction,
    new_direction: components::Direction,
) -> Events {
    *direction = new_direction;

    Events::default()
}
