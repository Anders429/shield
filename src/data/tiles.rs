use crate::{
    components::Palette,
    data::{colors, sprites},
    tile::Tile,
};

pub(crate) static TILES: [Tile; 2] = [
    Tile {
        sprite: &sprites::GRASS,
        palette: Palette {
            color_a: colors::SLIMEGREEN,
            color_b: colors::GREEN,
            color_c: colors::DARKGREEN,
        },
    },
    Tile {
        sprite: &sprites::FENCE,
        palette: Palette {
            color_a: colors::BROWN,
            color_b: colors::DARKBROWN,
            color_c: colors::NIGHTBLUE,
        },
    },
];

pub(crate) static GRASS: Tile = Tile {
    sprite: &sprites::GRASS,
    palette: Palette {
        color_a: colors::YELLOW,
        color_b: colors::SLIMEGREEN,
        color_c: colors::GREEN,
    },
};
