use crate::components;

pub(crate) struct Tile<'a> {
    pub(crate) sprite: &'a components::Sprite,
    pub(crate) palette: components::Palette,
}
