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

pub(crate) static FENCE_VERTICAL: Tile = Tile {
    sprite: &sprites::FENCE_VERTICAL,
    palette: Palette {
        color_a: colors::BROWN,
        color_b: colors::DARKBROWN,
        color_c: colors::NIGHTBLUE,
    },
};

pub(crate) static FENCE_CORNER_TOP_LEFT: Tile = Tile {
    sprite: &sprites::FENCE_CORNER_TOP_LEFT,
    palette: Palette {
        color_a: colors::BROWN,
        color_b: colors::DARKBROWN,
        color_c: colors::NIGHTBLUE,
    },
};

pub(crate) static FENCE_CORNER_TOP_RIGHT: Tile = Tile {
    sprite: &sprites::FENCE_CORNER_TOP_RIGHT,
    palette: Palette {
        color_a: colors::BROWN,
        color_b: colors::DARKBROWN,
        color_c: colors::NIGHTBLUE,
    },
};

pub(crate) static BUSH: Tile = Tile {
    sprite: &sprites::BUSH,
    palette: Palette {
        color_a: colors::BROWN,
        color_b: colors::DARKBROWN,
        color_c: colors::NIGHTBLUE,
    },
};
