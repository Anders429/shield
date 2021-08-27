use crate::components::sprite::{Sprite, SpriteSheet};

pub(crate) static PLAYER: SpriteSheet<1, 1> = SpriteSheet {
    up: [[[&super::sprites::PLAYER_UP_A]], [[&super::sprites::PLAYER_UP_B]]],
    right: [[[&super::sprites::PLAYER_RIGHT_A]], [[&super::sprites::PLAYER_RIGHT_B]]],
    down: [[[&super::sprites::PLAYER_DOWN_A]], [[&super::sprites::PLAYER_DOWN_B]]],
    left: [[[&super::sprites::PLAYER_LEFT_A]], [[&super::sprites::PLAYER_LEFT_B]]],
};
