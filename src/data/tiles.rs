use crate::{
    components::Palette,
    data::{colors, sprites},
    tile::Tile,
};

pub(crate) static GRASS: Tile = Tile {
    sprite: &sprites::GRASS,
    palette: Palette {
        color_a: colors::YELLOW,
        color_b: colors::SLIMEGREEN,
        color_c: colors::GREEN,
    },
};

pub(crate) static FENCE: Tile = Tile {
    sprite: &sprites::FENCE,
    palette: Palette {
        color_a: colors::BROWN,
        color_b: colors::DARKBROWN,
        color_c: colors::NIGHTBLUE,
    },
};
