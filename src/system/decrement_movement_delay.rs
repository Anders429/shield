use crate::{components, events::Events};

pub(crate) fn decrement_movement_delay(
    movement_delay: &mut components::MovementDelay,
    speed: components::Speed,
) -> Events {
    if *movement_delay == 0 {
        *movement_delay = speed;
    } else {
        *movement_delay -= 1;
    }

    Events::default()
}
