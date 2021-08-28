use bitflags::bitflags;

bitflags! {
    pub(crate) struct Entity: u32 {
        const POSITION = 0b0000_0000_0000_0000_0000_0000_0000_0001;
        const BOUNDING_BOX = 0b0000_0000_0000_0000_0000_0000_0000_0010;
        const FACING_DIRECTION = 0b0000_0000_0000_0000_0000_0000_0000_0100;
        const SPEED = 0b0000_0000_0000_0000_0000_0000_0000_1000;
        const ACCEPTS_INPUT = 0b0000_0000_0000_0000_0000_0000_0001_0000;
        const MOVEMENT_DELAY = 0b0000_0000_0000_0000_0000_0000_0010_0000;
        const CHUNK = 0b0000_0000_0000_0000_0000_0000_0100_0000;
        const SPRITESHEET_1X1 = 0b0000_0000_0000_0000_0000_0000_1000_0000;
        const PALETTE = 0b0000_0000_0000_0000_0000_0001_0000_0000;
        const STATIC_SPRITE = 0b0000_0000_0000_0000_0000_0010_0000_0000;
        const HEALTH_POINTS = 0b0000_0000_0000_0000_0000_0100_0000_0000;
        const PLAYER = 0b0000_0000_0000_0000_0000_1000_0000_0000;
        const WALKING_TIMER = 0b0000_0000_0000_0000_0001_0000_0000_0000;
        const WALKING = 0b0000_0000_0000_0000_0010_0000_0000_0000;
        const WALKING_ANIMATION_STATE = 0b0000_0000_0000_0000_0100_0000_0000_0000;
        const DAMAGE = 0b0000_0000_0000_0000_1000_0000_0000_0000;
        const IMMOVABLE = 0b0000_0000_0000_0001_0000_0000_0000_0000;
        const MOVING_DIRECTION = 0b0000_0000_0000_0010_0000_0000_0000_0000;
        const DAMAGE_INVULNERABILITY_TIMER = 0b0000_0000_0000_0100_0000_0000_0000_0000;
        const HOLDING = 0b0000_0000_0000_1000_0000_0000_0000_0000;
        const GENERATION = 0b0000_0000_0001_0000_0000_0000_0000_0000;
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

    pub(crate) fn facing_direction() -> Self {
        Self::FACING_DIRECTION
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

    pub(crate) fn walking_timer() -> Self {
        Self::WALKING_TIMER
    }

    pub(crate) fn walking() -> Self {
        Self::WALKING
    }

    pub(crate) fn walking_animation_state() -> Self {
        Self::WALKING_ANIMATION_STATE
    }

    pub(crate) fn damage() -> Self {
        Self::DAMAGE
    }

    pub(crate) fn immovable() -> Self {
        Self::IMMOVABLE
    }

    pub(crate) fn moving_direction() -> Self {
        Self::MOVING_DIRECTION
    }

    pub(crate) fn damage_invulnerability_timer() -> Self {
        Self::DAMAGE_INVULNERABILITY_TIMER
    }

    pub(crate) fn holding() -> Self {
        Self::HOLDING
    }

    pub(crate) fn generation() -> Self {
        Self::GENERATION
    }

    pub(crate) fn has_position(&self) -> bool {
        self.contains(Self::POSITION)
    }

    pub(crate) fn has_bounding_box(&self) -> bool {
        self.contains(Self::BOUNDING_BOX)
    }

    pub(crate) fn has_facing_direction(&self) -> bool {
        self.contains(Self::FACING_DIRECTION)
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

    pub(crate) fn has_walking_timer(&self) -> bool {
        self.contains(Self::WALKING_TIMER)
    }

    pub(crate) fn is_walking(&self) -> bool {
        self.contains(Self::WALKING)
    }

    pub(crate) fn has_walking_animation_state(&self) -> bool {
        self.contains(Self::WALKING_ANIMATION_STATE)
    }

    pub(crate) fn has_damage(&self) -> bool {
        self.contains(Self::DAMAGE)
    }

    pub(crate) fn is_immovable(&self) -> bool {
        self.contains(Self::IMMOVABLE)
    }

    pub(crate) fn has_moving_direction(&self) -> bool {
        self.contains(Self::MOVING_DIRECTION)
    }

    pub(crate) fn has_damage_invulnerability_timer(&self) -> bool {
        self.contains(Self::DAMAGE_INVULNERABILITY_TIMER)
    }

    pub(crate) fn has_holding(&self) -> bool {
        self.contains(Self::HOLDING)
    }

    pub(crate) fn has_generation(&self) -> bool {
        self.contains(Self::GENERATION)
    }

    pub(crate) fn remove_walking(&mut self) {
        self.remove(Self::WALKING)
    }

    pub(crate) fn remove_moving_direction(&mut self) {
        self.remove(Self::MOVING_DIRECTION)
    }

    pub(crate) fn remove_damage_invulnerability_timer(&mut self) {
        self.remove(Self::DAMAGE_INVULNERABILITY_TIMER)
    }

    pub(crate) fn remove_holding(&mut self) {
        self.remove(Self::HOLDING)
    }
}
