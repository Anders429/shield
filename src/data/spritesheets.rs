use crate::components::sprite::{Sprite, SpriteSheet};

pub(crate) static PLAYER: SpriteSheet<1, 1> = SpriteSheet {
    up: [
        [[&super::sprites::PLAYER_UP_A]],
        [[&super::sprites::PLAYER_UP_B]],
        [[&super::sprites::PLAYER_UP_C]],
    ],
    right: [
        [[&super::sprites::PLAYER_RIGHT_A]],
        [[&super::sprites::PLAYER_RIGHT_B]],
        [[&super::sprites::PLAYER_RIGHT_C]],
    ],
    down: [
        [[&super::sprites::PLAYER_DOWN_A]],
        [[&super::sprites::PLAYER_DOWN_B]],
        [[&super::sprites::PLAYER_DOWN_C]],
    ],
    left: [
        [[&super::sprites::PLAYER_LEFT_A]],
        [[&super::sprites::PLAYER_LEFT_B]],
        [[&super::sprites::PLAYER_LEFT_C]],
    ],
};

pub(crate) static SHIELD: SpriteSheet<1, 1> = SpriteSheet {
    up: [
        [[&super::sprites::SHIELD_UP]],
        [[&super::sprites::SHIELD_UP]],
        [[&super::sprites::SHIELD_UP]],
    ],
    right: [
        [[&super::sprites::SHIELD_DOWN]],
        [[&super::sprites::SHIELD_DOWN]],
        [[&super::sprites::SHIELD_DOWN]],
    ],
    down: [
        [[&super::sprites::SHIELD_DOWN]],
        [[&super::sprites::SHIELD_DOWN]],
        [[&super::sprites::SHIELD_DOWN]],
    ],
    left: [
        [[&super::sprites::SHIELD_DOWN]],
        [[&super::sprites::SHIELD_DOWN]],
        [[&super::sprites::SHIELD_DOWN]],
    ],
};
