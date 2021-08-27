use crate::{components, events::Events, World};

pub(crate) fn collisions<const ENTITY_COUNT: usize>(world: &mut World<ENTITY_COUNT>) -> Events {
    Events::default()
}