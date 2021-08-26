use bitflags::bitflags;

bitflags! {
    pub(crate) struct Entity: u16 {
        const POSITION = 0b0000_0000_0000_0001;
        const BOUNDING_BOX = 0b0000_0000_0000_0010;
        const DIRECTION = 0b0000_0000_0000_0100;
        const SPEED = 0b0000_0000_0000_1000;
        const ACCEPTS_INPUT = 0b0000_0000_0001_0000;
        const MOVEMENT_DELAY = 0b0000_0000_0010_0000;
        const CHUNK = 0b0000_0000_0100_0000;
        const SPRITESHEET_1X1 = 0b0000_0000_1000_0000;
        const PALETTE = 0b0000_0001_0000_0000;
        const STATIC_SPRITE = 0b0000_0010_0000_0000;
        const HEALTH_POINTS = 0b0000_0100_0000_0000;
        const PLAYER = 0b0000_1000_0000_0000;
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self::empty()
    }
}

impl Entity {
    pub(crate) fn position() -> Self {
        Self::POSITION
    }

    pub(crate) fn bounding_box() -> Self {
        Self::BOUNDING_BOX
    }

    pub(crate) fn direction() -> Self {
        Self::DIRECTION
    }

    pub(crate) fn speed() -> Self {
        Self::SPEED
    }

    pub(crate) fn accepts_input() -> Self {
        Self::ACCEPTS_INPUT
    }

    pub(crate) fn movement_delay() -> Self {
        Self::MOVEMENT_DELAY
    }

    pub(crate) fn chunk() -> Self {
        Self::CHUNK
    }

    pub(crate) fn spritesheet_1x1() -> Self {
        Self::SPRITESHEET_1X1
    }

    pub(crate) fn palette() -> Self {
        Self::PALETTE
    }

    pub(crate) fn static_sprite() -> Self {
        Self::STATIC_SPRITE
    }

    pub(crate) fn health_points() -> Self {
        Self::HEALTH_POINTS
    }

    pub(crate) fn player() -> Self {
        Self::PLAYER
    }

    pub(crate) fn has_position(&self) -> bool {
        self.contains(Self::POSITION)
    }

    pub(crate) fn has_bounding_box(&self) -> bool {
        self.contains(Self::BOUNDING_BOX)
    }

    pub(crate) fn has_direction(&self) -> bool {
        self.contains(Self::DIRECTION)
    }

    pub(crate) fn has_speed(&self) -> bool {
        self.contains(Self::SPEED)
    }

    pub(crate) fn has_accepts_input(&self) -> bool {
        self.contains(Self::ACCEPTS_INPUT)
    }

    pub(crate) fn has_movement_delay(&self) -> bool {
        self.contains(Self::MOVEMENT_DELAY)
    }

    pub(crate) fn has_chunk(&self) -> bool {
        self.contains(Self::CHUNK)
    }

    pub(crate) fn has_spritesheet_1x1(&self) -> bool {
        self.contains(Self::SPRITESHEET_1X1)
    }

    pub(crate) fn has_palette(&self) -> bool {
        self.contains(Self::PALETTE)
    }

    pub(crate) fn has_static_sprite(&self) -> bool {
        self.contains(Self::STATIC_SPRITE)
    }

    pub(crate) fn has_health_points(&self) -> bool {
        self.contains(Self::HEALTH_POINTS)
    }

    pub(crate) fn is_player(&self) -> bool {
        self.contains(Self::PLAYER)
    }
}
