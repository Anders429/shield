use crate::components::sprite::{Sprite, SpriteSheet};

pub(crate) static PLAYER: SpriteSheet<1, 1> = SpriteSheet {
    // up: [[Sprite {
    //     rows: [0b0001_1011_0001_1011_0001_1011_0001_1011; 16],
    // }]],
    up: [[&super::sprites::GRASS]],
};
