use crate::components::sprite::Sprite;

pub(crate) static HEART: Sprite = Sprite {
    rows: [
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_11_11_11_00_00_11_11_11_00_00_00_00,
        0b00_00_00_11_10_01_01_11_11_01_01_01_11_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_01_01_11_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_01_11_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_01_11_00_00_00,
        0b00_00_00_00_11_10_10_10_10_10_01_11_00_00_00_00,
        0b00_00_00_00_00_11_10_10_10_01_11_00_00_00_00_00,
        0b00_00_00_00_00_00_11_10_10_11_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_11_11_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
    ],
};

pub(crate) static PLAYER_UP_A: Sprite = Sprite {
    rows: [
        0b00_00_00_00_11_11_11_11_11_11_11_11_00_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_10_11_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_10_11_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_10_11_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_10_11_00_00_00,
        0b00_00_11_11_11_10_10_10_10_10_10_11_11_11_00_00,
        0b00_11_10_10_11_11_11_11_11_11_11_11_10_10_11_00,
        0b00_11_10_10_10_10_10_10_10_10_10_10_10_10_11_00,
        0b11_11_11_10_10_10_10_10_10_10_10_10_10_10_11_11,
        0b11_01_11_10_10_10_10_10_10_10_10_10_10_11_01_11,
        0b11_01_11_10_10_10_10_10_10_10_10_10_10_11_01_11,
        0b00_11_11_11_11_11_11_11_11_11_11_11_11_11_11_00,
        0b00_00_11_10_10_10_10_10_10_10_10_10_10_11_00_00,
        0b00_00_00_11_11_11_11_11_11_11_11_11_11_00_00_00,
        0b00_00_00_11_01_01_11_00_00_11_01_01_11_00_00_00,
        0b00_00_00_11_11_11_11_00_00_11_11_11_11_00_00_00,
    ],
};

pub(crate) static PLAYER_UP_B: Sprite = Sprite {
    rows: [
        0b00_00_00_00_11_11_11_11_11_11_11_11_00_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_10_11_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_10_11_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_10_11_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_10_11_00_00_00,
        0b00_00_11_11_11_10_10_10_10_10_10_11_11_11_00_00,
        0b00_11_10_10_11_11_11_11_11_11_11_11_10_10_11_00,
        0b00_11_10_10_10_10_10_10_10_10_10_10_10_10_11_00,
        0b11_11_11_10_10_10_10_10_10_10_10_10_10_10_11_00,
        0b11_01_01_11_10_10_10_10_10_10_10_10_10_11_01_11,
        0b11_01_01_11_10_10_10_10_10_10_10_10_10_11_11_00,
        0b00_11_11_11_11_11_11_11_11_11_11_11_11_11_00_00,
        0b00_00_11_10_10_10_10_10_10_10_10_10_10_11_00_00,
        0b00_00_00_11_11_11_11_11_11_11_11_11_11_00_00_00,
        0b00_00_00_11_11_11_11_00_11_01_01_01_11_00_00_00,
        0b00_00_00_00_00_00_00_00_11_11_11_11_11_00_00_00,
    ],
};

pub(crate) static PLAYER_UP_C: Sprite = Sprite {
    rows: [
        0b00_00_00_00_11_11_11_11_11_11_11_11_00_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_10_11_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_10_11_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_10_11_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_10_11_00_00_00,
        0b00_00_11_11_11_10_10_10_10_10_10_11_11_11_00_00,
        0b00_11_10_10_11_11_11_11_11_11_11_11_10_10_11_00,
        0b00_11_10_10_10_10_10_10_10_10_10_10_10_10_11_00,
        0b00_11_11_10_10_10_10_10_10_10_10_10_10_11_11_11,
        0b11_01_11_10_10_10_10_10_10_10_10_10_11_01_01_11,
        0b00_11_11_10_10_10_10_10_10_10_10_10_11_01_01_11,
        0b00_00_11_11_11_11_11_11_11_11_11_11_11_11_11_00,
        0b00_00_11_10_10_10_10_10_10_10_10_10_10_11_00_00,
        0b00_00_00_11_11_11_11_11_11_11_11_11_11_00_00_00,
        0b00_00_00_11_01_01_01_11_00_11_11_11_11_00_00_00,
        0b00_00_00_11_11_11_11_11_00_00_00_00_00_00_00_00,
    ],
};

pub(crate) static PLAYER_RIGHT_A: Sprite = Sprite {
    rows: [
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
    ],
};

pub(crate) static PLAYER_RIGHT_B: Sprite = Sprite {
    rows: [
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
    ],
};

pub(crate) static PLAYER_RIGHT_C: Sprite = Sprite {
    rows: [
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
    ],
};

pub(crate) static PLAYER_DOWN_A: Sprite = Sprite {
    rows: [
        0b00_00_00_00_11_11_11_11_11_11_11_11_00_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_10_11_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_10_11_00_00_00,
        0b00_00_00_11_10_11_11_10_10_11_11_10_11_00_00_00,
        0b00_00_00_11_11_01_01_11_11_01_01_11_11_00_00_00,
        0b00_00_11_11_11_01_11_01_01_11_01_11_11_11_00_00,
        0b00_11_10_10_11_01_01_01_01_01_01_11_10_10_11_00,
        0b00_11_10_10_10_11_01_01_01_01_11_10_10_10_11_00,
        0b11_11_11_10_10_10_11_11_11_11_10_10_10_11_11_11,
        0b11_01_01_11_10_10_10_10_10_10_10_10_11_01_01_11,
        0b11_01_01_11_10_10_10_10_10_10_10_10_11_01_01_11,
        0b00_11_11_11_11_11_11_11_11_11_11_11_11_11_11_00,
        0b00_00_11_10_10_10_10_10_10_10_10_10_10_11_00_00,
        0b00_00_00_11_11_11_11_11_11_11_11_11_11_00_00_00,
        0b00_00_00_11_01_01_11_00_00_11_01_01_11_00_00_00,
        0b00_00_00_11_11_11_11_00_00_11_11_11_11_00_00_00,
    ],
};

pub(crate) static PLAYER_DOWN_B: Sprite = Sprite {
    rows: [
        0b00_00_00_00_11_11_11_11_11_11_11_11_00_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_10_11_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_10_11_00_00_00,
        0b00_00_00_11_10_11_11_10_10_11_11_10_11_00_00_00,
        0b00_00_00_11_11_01_01_11_11_01_01_11_11_00_00_00,
        0b00_00_11_11_11_01_11_01_01_11_01_11_11_11_00_00,
        0b00_11_10_10_11_01_01_01_01_01_01_11_10_10_11_00,
        0b00_11_10_10_10_11_01_01_01_01_11_10_10_11_11_11,
        0b11_11_11_10_10_10_11_11_11_11_10_10_11_01_01_11,
        0b11_01_11_10_10_10_10_10_10_10_10_10_11_01_01_11,
        0b11_11_11_10_10_10_10_10_10_10_10_10_11_11_11_00,
        0b00_00_11_11_11_11_11_11_11_11_11_11_11_11_00_00,
        0b00_00_11_10_10_10_10_10_10_10_10_10_10_11_00_00,
        0b00_00_11_11_11_11_11_11_11_11_11_11_11_00_00_00,
        0b00_00_00_11_01_01_01_11_00_11_11_11_11_00_00_00,
        0b00_00_00_11_11_11_11_11_00_00_00_00_00_00_00_00,
    ],
};

pub(crate) static PLAYER_DOWN_C: Sprite = Sprite {
    rows: [
        0b00_00_00_00_11_11_11_11_11_11_11_11_00_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_10_11_00_00_00,
        0b00_00_00_11_10_10_10_10_10_10_10_10_11_00_00_00,
        0b00_00_00_11_10_11_11_10_10_11_11_10_11_00_00_00,
        0b00_00_00_11_11_01_01_11_11_01_01_11_11_00_00_00,
        0b00_00_11_11_11_01_11_01_01_11_01_11_11_11_00_00,
        0b00_11_10_10_11_01_01_01_01_01_01_11_10_10_11_00,
        0b11_11_11_10_10_11_01_01_01_01_11_10_10_10_11_00,
        0b11_01_01_11_10_10_11_11_11_11_10_10_10_11_11_11,
        0b11_01_01_11_10_10_10_10_10_10_10_10_10_11_01_11,
        0b00_11_11_11_10_10_10_10_10_10_10_10_10_11_11_11,
        0b00_00_11_11_11_11_11_11_11_11_11_11_11_11_00_00,
        0b00_00_11_10_10_10_10_10_10_10_10_10_10_11_00_00,
        0b00_00_00_11_11_11_11_11_11_11_11_11_11_11_00_00,
        0b00_00_00_11_11_11_11_00_11_01_01_01_11_00_00_00,
        0b00_00_00_00_00_00_00_00_11_11_11_11_11_00_00_00,
    ],
};

pub(crate) static PLAYER_LEFT_A: Sprite = Sprite {
    rows: [
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
    ],
};

pub(crate) static PLAYER_LEFT_B: Sprite = Sprite {
    rows: [
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
    ],
};

pub(crate) static PLAYER_LEFT_C: Sprite = Sprite {
    rows: [
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
        0b00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00,
    ],
};

pub(crate) static SHIELD_UP: Sprite = Sprite {
    rows: [
        0b00_00_00_00_11_11_11_11_11_11_11_11_00_00_00_00,
        0b11_11_11_11_01_01_01_01_01_01_01_01_11_11_11_11,
        0b11_01_01_01_01_01_01_01_01_01_01_01_01_01_01_11,
        0b11_01_01_01_01_01_01_01_01_01_01_01_01_01_01_11,
        0b11_01_01_01_01_01_01_01_01_01_01_01_01_01_01_11,
        0b11_01_01_01_01_01_01_01_01_01_01_01_01_01_01_11,
        0b00_11_01_01_01_01_01_01_01_01_01_01_01_01_11_00,
        0b00_11_01_01_01_01_01_01_01_01_01_01_01_01_11_00,
        0b00_11_01_01_01_01_01_01_01_01_01_01_01_01_11_00,
        0b00_00_11_01_01_01_01_01_01_01_01_01_01_11_00_00,
        0b00_00_11_01_01_01_01_01_01_01_01_01_01_11_00_00,
        0b00_00_11_01_01_01_01_01_01_01_01_01_01_11_00_00,
        0b00_00_00_11_01_01_01_01_01_01_01_01_11_00_00_00,
        0b00_00_00_11_01_01_01_01_01_01_01_01_11_00_00_00,
        0b00_00_00_00_11_01_01_01_01_01_01_11_00_00_00_00,
        0b00_00_00_00_00_11_11_11_11_11_11_00_00_00_00_00,
    ],
};

pub(crate) static SHIELD_DOWN: Sprite = Sprite {
    rows: [
        0b00_00_00_00_11_11_11_11_11_11_11_11_00_00_00_00,
        0b11_11_11_11_01_01_01_11_11_01_01_01_11_11_11_11,
        0b11_01_01_01_01_01_01_11_11_01_01_01_01_01_01_11,
        0b11_01_01_01_01_01_01_11_11_01_01_01_01_01_01_11,
        0b11_01_01_01_01_01_10_10_10_10_01_01_01_01_01_11,
        0b11_11_11_11_11_11_10_10_10_10_11_11_11_11_11_11,
        0b00_11_11_11_11_11_10_10_10_10_11_11_11_11_11_00,
        0b00_11_01_01_01_01_10_10_10_10_01_01_01_01_11_00,
        0b00_11_01_01_01_01_01_11_11_01_01_01_01_01_11_00,
        0b00_00_11_01_01_01_01_11_11_01_01_01_01_11_00_00,
        0b00_00_11_01_01_01_01_11_11_01_01_01_01_11_00_00,
        0b00_00_11_01_01_01_01_11_11_01_01_01_01_11_00_00,
        0b00_00_00_11_01_01_01_11_11_01_01_01_11_00_00_00,
        0b00_00_00_11_01_01_01_11_11_01_01_01_11_00_00_00,
        0b00_00_00_00_11_01_01_11_11_01_01_11_00_00_00_00,
        0b00_00_00_00_00_11_11_11_11_11_11_00_00_00_00_00,
    ],
};

pub(crate) static GRASS: Sprite = Sprite {
    rows: [
        0b11_11_11_11_10_11_11_11_11_11_11_11_11_11_11_11,
        0b10_11_11_10_11_11_11_11_11_11_11_11_11_11_11_11,
        0b11_10_11_10_11_11_11_11_11_11_11_11_11_11_11_11,
        0b11_10_11_11_11_11_11_11_11_11_11_11_11_11_11_11,
        0b11_11_11_11_11_11_11_11_01_11_11_11_01_11_11_11,
        0b11_11_11_11_11_11_11_11_11_01_11_01_11_11_11_11,
        0b11_11_11_11_11_11_11_11_11_11_11_01_11_11_11_11,
        0b11_11_11_11_11_11_11_11_11_11_11_11_11_11_11_11,
        0b11_11_11_11_11_11_11_11_11_11_11_11_11_11_11_11,
        0b11_11_11_11_01_11_11_11_11_11_11_11_11_11_11_11,
        0b11_11_11_11_11_01_11_11_01_11_11_11_11_11_11_11,
        0b11_11_11_11_11_01_11_01_11_11_11_11_11_11_11_11,
        0b11_11_11_11_11_11_11_11_11_11_11_11_10_11_11_11,
        0b11_11_11_11_11_11_11_11_11_11_11_11_11_10_11_11,
        0b11_11_11_11_11_11_11_11_11_11_11_11_11_10_11_11,
        0b11_11_11_11_11_11_11_11_11_11_11_11_11_11_11_11,
    ],
};

pub(crate) static FENCE: Sprite = Sprite {
    rows: [
        0b0000_0000_0000_0011_1100_0000_0000_0000,
        0b0000_0000_0000_1101_0111_0000_0000_0000,
        0b0000_0000_0011_0101_0101_1100_0000_0000,
        0b0000_0000_0011_1010_1010_1100_0000_0000,
        0b0000_0000_0011_1010_1010_1100_0000_0000,
        0b1111_1111_1111_1111_1111_1111_1111_1111,
        0b0101_0101_0101_0110_1010_1010_1001_0101,
        0b1010_1010_1010_1010_1010_1010_1010_1010,
        0b1010_1010_1010_1010_1010_1010_1010_1010,
        0b1010_1010_1010_1010_1010_1010_1010_1010,
        0b1111_1111_1111_1111_1111_1111_1111_1111,
        0b0000_0000_0011_1010_1010_1100_0000_0000,
        0b0000_0000_0011_1010_1010_1100_0000_0000,
        0b0000_0000_0011_0101_0101_1100_0000_0000,
        0b0000_0000_0011_0101_0110_1100_0000_0000,
        0b0000_0000_0011_1111_1111_1100_0000_0000,
    ],
};
