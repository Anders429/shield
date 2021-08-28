#![feature(unboxed_closures)]

pub mod constants;
pub mod data;

mod chunk;
mod components;
mod entity;
mod events;
mod generational_index;
mod system;
mod tile;

use by_address::ByAddress;
use entity::Entity;
use events::{Events, Input};
use generational_index::GenerationalIndex;
use itertools::izip;
use lru::LruCache;
use sdl2::{
    render::{Canvas, RenderTarget, Texture, TextureCreator},
    EventPump,
};
use tile::Tile;

/// Singleton components for the world state.
#[derive(Default)]
struct Resources {
    chunk: components::Chunk,
    position: components::Position,
    game_state: components::GameState,
}

struct Components<'a, const ENTITY_COUNT: usize> {
    pub(crate) positions: Box<[components::Position; ENTITY_COUNT]>,
    pub(crate) bounding_boxes: Box<[components::BoundingBox; ENTITY_COUNT]>,
    pub(crate) facing_directions: Box<[components::Direction; ENTITY_COUNT]>,
    pub(crate) speeds: Box<[components::Speed; ENTITY_COUNT]>,
    pub(crate) accepts_input: Box<[components::AcceptsInput; ENTITY_COUNT]>,
    pub(crate) movement_delays: Box<[components::MovementDelay; ENTITY_COUNT]>,
    pub(crate) chunks: Box<[components::Chunk; ENTITY_COUNT]>,
    pub(crate) spritesheets_1x1: Box<[Option<&'a components::SpriteSheet<'a, 1, 1>>; ENTITY_COUNT]>,
    pub(crate) palettes: Box<[components::Palette; ENTITY_COUNT]>,
    pub(crate) static_sprites: Box<[Option<&'a components::Sprite>; ENTITY_COUNT]>,
    pub(crate) health_points: Box<[components::HealthPoints; ENTITY_COUNT]>,
    pub(crate) walking_timers: Box<[components::Timer; ENTITY_COUNT]>,
    pub(crate) walking_animation_states: Box<[components::WalkingAnimationState; ENTITY_COUNT]>,
    pub(crate) damages: Box<[components::Damage; ENTITY_COUNT]>,
    pub(crate) moving_directions: Box<[components::Direction; ENTITY_COUNT]>,
    pub(crate) damage_invulnerability_timers: Box<[components::Timer; ENTITY_COUNT]>,
    pub(crate) holdings: Box<[components::EntityReference; ENTITY_COUNT]>,
    pub(crate) generations: Box<[components::Generation; ENTITY_COUNT]>,
    pub(crate) helds: Box<[components::EntityReference; ENTITY_COUNT]>,
    pub(crate) grabs: Box<[components::EntityReference; ENTITY_COUNT]>,
    pub(crate) usables: Box<[components::Usable; ENTITY_COUNT]>,
    pub(crate) use_cooldowns: Box<[components::Timer; ENTITY_COUNT]>,
    pub(crate) retreatings: Box<[components::Timer; ENTITY_COUNT]>,
}

impl<const ENTITY_COUNT: usize> Default for Components<'_, ENTITY_COUNT> {
    fn default() -> Self {
        Self {
            positions: Box::new([components::Position::default(); ENTITY_COUNT]),
            bounding_boxes: Box::new([components::BoundingBox::default(); ENTITY_COUNT]),
            facing_directions: Box::new([components::Direction::default(); ENTITY_COUNT]),
            speeds: Box::new([components::Speed::default(); ENTITY_COUNT]),
            accepts_input: Box::new([components::AcceptsInput::default(); ENTITY_COUNT]),
            movement_delays: Box::new([components::MovementDelay::default(); ENTITY_COUNT]),
            chunks: Box::new([components::Chunk::default(); ENTITY_COUNT]),
            spritesheets_1x1: Box::new([None; ENTITY_COUNT]),
            palettes: Box::new([components::Palette::default(); ENTITY_COUNT]),
            static_sprites: Box::new([None; ENTITY_COUNT]),
            health_points: Box::new([components::HealthPoints::default(); ENTITY_COUNT]),
            walking_timers: Box::new([components::Timer::default(); ENTITY_COUNT]),
            walking_animation_states: Box::new(
                [components::WalkingAnimationState::default(); ENTITY_COUNT],
            ),
            damages: Box::new([components::Damage::default(); ENTITY_COUNT]),
            moving_directions: Box::new([components::Direction::default(); ENTITY_COUNT]),
            damage_invulnerability_timers: Box::new([components::Timer::default(); ENTITY_COUNT]),
            holdings: Box::new([components::EntityReference::default(); ENTITY_COUNT]),
            generations: Box::new([components::Generation::default(); ENTITY_COUNT]),
            helds: Box::new([components::EntityReference::default(); ENTITY_COUNT]),
            grabs: Box::new([components::EntityReference::default(); ENTITY_COUNT]),
            usables: Box::new([components::Usable::default(); ENTITY_COUNT]),
            use_cooldowns: Box::new([components::Timer::default(); ENTITY_COUNT]),
            retreatings: Box::new([components::Timer::default(); ENTITY_COUNT]),
        }
    }
}

pub struct World<'a, const ENTITY_COUNT: usize> {
    pub(crate) generational_index_allocator: generational_index::Allocator<ENTITY_COUNT>,

    pub(crate) entities: Box<[Entity; ENTITY_COUNT]>,

    pub(crate) resources: Resources,
    pub(crate) components: Components<'a, ENTITY_COUNT>,
}

impl<const ENTITY_COUNT: usize> Default for World<'_, ENTITY_COUNT> {
    fn default() -> Self {
        Self {
            generational_index_allocator: generational_index::Allocator::default(),

            entities: Box::new([Entity::default(); ENTITY_COUNT]),

            resources: Resources::default(),
            components: Components::default(),
        }
    }
}

impl<'a, const ENTITY_COUNT: usize> World<'a, ENTITY_COUNT> {
    pub fn new_game(&mut self) -> anyhow::Result<()> {
        // Reset the game state.
        *self = Self::default();

        if let None = self.register_player() {
            return Err(anyhow::format_err!("Unable to register player."));
        }
        if let None = self.register_shield() {
            return Err(anyhow::format_err!("Unable to register shield."));
        }
        if let None = self.register_slime(constants::STARTING_POSITION, constants::STARTING_CHUNK) {
            return Err(anyhow::format_err!("Unable to register slime."));
        }
        self.resources.chunk = constants::STARTING_CHUNK;
        self.resources.position = constants::STARTING_POSITION;

        // TEMPORARY
        for (x, y, tile) in data::chunks::CHUNK
            .layer_1
            .tiles
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, tile)| match tile {
                        Some(unwrapped_tile) => Some((j, i, unwrapped_tile)),
                        None => None,
                    })
            })
            .flatten()
        {
            self.register_tile(
                tile,
                components::Position {
                    x: x as u16 * 16,
                    y: y as u16 * 16,
                },
                constants::STARTING_CHUNK,
            );
        }

        for (x, y, tile) in data::chunks::CHUNK
            .layer_2
            .tiles
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, tile)| match tile {
                        Some(unwrapped_tile) => Some((j, i, unwrapped_tile)),
                        None => None,
                    })
            })
            .flatten()
        {
            if let Some(generational_index) = self.register_tile(
                tile,
                components::Position {
                    x: x as u16 * 16,
                    y: y as u16 * 16,
                },
                constants::STARTING_CHUNK,
            ) {
                // TEMPORARY
                unsafe {
                    *self.entities.get_unchecked_mut(generational_index.index) |=
                        Entity::bounding_box() | Entity::immovable();

                    *self
                        .components
                        .bounding_boxes
                        .get_unchecked_mut(generational_index.index) = components::BoundingBox {
                        width: 16,
                        height: 16,
                        offset_x: 0,
                        offset_y: 0,
                    };
                }
            }
        }

        Ok(())
    }

    fn register_player(&mut self) -> Option<GenerationalIndex> {
        let generational_index = self.generational_index_allocator.allocate()?;

        unsafe {
            // SAFETY: The index here will always be within the bounds of ENTITY_COUNT.
            *self.entities.get_unchecked_mut(generational_index.index) = Entity::position()
                | Entity::bounding_box()
                | Entity::facing_direction()
                | Entity::speed()
                | Entity::accepts_input()
                | Entity::chunk()
                | Entity::spritesheet_1x1()
                | Entity::palette()
                | Entity::health_points()
                | Entity::player()
                | Entity::walking_timer()
                | Entity::walking_animation_state()
                | Entity::generation()
                | Entity::moving_direction();

            *self
                .components
                .positions
                .get_unchecked_mut(generational_index.index) = constants::STARTING_POSITION;
            *self
                .components
                .bounding_boxes
                .get_unchecked_mut(generational_index.index) = components::BoundingBox {
                width: 16,
                height: 12,
                offset_x: 0,
                offset_y: 4,
            };
            *self
                .components
                .facing_directions
                .get_unchecked_mut(generational_index.index) = components::Direction::Down;
            *self
                .components
                .speeds
                .get_unchecked_mut(generational_index.index) = 1;
            *self
                .components
                .accepts_input
                .get_unchecked_mut(generational_index.index) = components::AcceptsInput::Player;
            *self
                .components
                .chunks
                .get_unchecked_mut(generational_index.index) = constants::STARTING_CHUNK;
            *self
                .components
                .spritesheets_1x1
                .get_unchecked_mut(generational_index.index) = Some(&data::spritesheets::PLAYER);
            *self
                .components
                .palettes
                .get_unchecked_mut(generational_index.index) = components::sprite::Palette {
                color_a: data::colors::ORANGE,
                color_b: data::colors::SKYBLUE,
                color_c: data::colors::VOID,
            };
            *self
                .components
                .health_points
                .get_unchecked_mut(generational_index.index) =
                components::HealthPoints { current: 3, max: 3 };
            *self
                .components
                .walking_timers
                .get_unchecked_mut(generational_index.index) = 0;
            *self
                .components
                .walking_animation_states
                .get_unchecked_mut(generational_index.index) =
                components::WalkingAnimationState::StandingA;
            *self
                .components
                .generations
                .get_unchecked_mut(generational_index.index) = generational_index.generation;
            *self
                .components
                .moving_directions
                .get_unchecked_mut(generational_index.index) = components::Direction::Down;
        }

        Some(generational_index)
    }

    pub(crate) fn register_shield(&mut self) -> Option<GenerationalIndex> {
        let generational_index = self.generational_index_allocator.allocate()?;

        unsafe {
            // SAFETY: The index here will always be within the bounds of ENTITY_COUNT.
            *self.entities.get_unchecked_mut(generational_index.index) = Entity::position()
                | Entity::bounding_box()
                | Entity::facing_direction()
                | Entity::chunk()
                | Entity::spritesheet_1x1()
                | Entity::palette()
                | Entity::generation()
                | Entity::holdable()
                | Entity::usable();

            *self
                .components
                .positions
                .get_unchecked_mut(generational_index.index) = constants::STARTING_POSITION;
            *self
                .components
                .bounding_boxes
                .get_unchecked_mut(generational_index.index) = components::BoundingBox {
                width: 16,
                height: 16,
                offset_x: 0,
                offset_y: 0,
            };
            *self
                .components
                .facing_directions
                .get_unchecked_mut(generational_index.index) = components::Direction::Down;
            *self
                .components
                .chunks
                .get_unchecked_mut(generational_index.index) = constants::STARTING_CHUNK;
            *self
                .components
                .spritesheets_1x1
                .get_unchecked_mut(generational_index.index) = Some(&data::spritesheets::SHIELD);
            *self
                .components
                .palettes
                .get_unchecked_mut(generational_index.index) = components::sprite::Palette {
                color_a: data::colors::BROWN,
                color_b: data::colors::SKYBLUE,
                color_c: data::colors::VOID,
            };
            *self
                .components
                .generations
                .get_unchecked_mut(generational_index.index) = generational_index.generation;
            *self
                .components
                .usables
                .get_unchecked_mut(generational_index.index) = components::Usable::Shield;
        }

        Some(generational_index)
    }

    pub(crate) fn register_slime(
        &mut self,
        position: components::Position,
        chunk: components::Chunk,
    ) -> Option<GenerationalIndex> {
        let generational_index = self.generational_index_allocator.allocate()?;

        unsafe {
            *self.entities.get_unchecked_mut(generational_index.index) = Entity::position()
                | Entity::chunk()
                | Entity::spritesheet_1x1()
                | Entity::palette()
                | Entity::facing_direction()
                | Entity::health_points()
                | Entity::bounding_box()
                | Entity::damage()
                | Entity::generation()
                | Entity::facing_direction()
                | Entity::accepts_input()
                | Entity::moving_direction();

            *self
                .components
                .positions
                .get_unchecked_mut(generational_index.index) = position;
            *self
                .components
                .chunks
                .get_unchecked_mut(generational_index.index) = chunk;
            *self
                .components
                .spritesheets_1x1
                .get_unchecked_mut(generational_index.index) = Some(&data::spritesheets::SLIME);
            *self
                .components
                .palettes
                .get_unchecked_mut(generational_index.index) = components::sprite::Palette {
                color_a: data::colors::MEAT,
                color_b: data::colors::RED,
                color_c: data::colors::VOID,
            };
            *self
                .components
                .facing_directions
                .get_unchecked_mut(generational_index.index) = components::Direction::Down;
            *self
                .components
                .health_points
                .get_unchecked_mut(generational_index.index) =
                components::HealthPoints { current: 2, max: 2 };
            *self
                .components
                .bounding_boxes
                .get_unchecked_mut(generational_index.index) = components::BoundingBox {
                width: 16,
                height: 16,
                offset_x: 0,
                offset_y: 0,
            };
            *self
                .components
                .damages
                .get_unchecked_mut(generational_index.index) = 1;
            *self
                .components
                .generations
                .get_unchecked_mut(generational_index.index) = generational_index.generation;
            *self
                .components
                .facing_directions
                .get_unchecked_mut(generational_index.index) = components::Direction::Down;
            *self
                .components
                .accepts_input
                .get_unchecked_mut(generational_index.index) =
                components::AcceptsInput::FollowsPlayer;
            *self
                .components
                .moving_directions
                .get_unchecked_mut(generational_index.index) = components::Direction::Down;
        }

        Some(generational_index)
    }

    pub(crate) fn register_tile(
        &mut self,
        tile: &'a Tile<'a>,
        position: components::Position,
        chunk: components::Chunk,
    ) -> Option<GenerationalIndex> {
        let generational_index = self.generational_index_allocator.allocate()?;

        unsafe {
            *self.entities.get_unchecked_mut(generational_index.index) =
                Entity::position() | Entity::chunk() | Entity::static_sprite() | Entity::palette();

            *self
                .components
                .positions
                .get_unchecked_mut(generational_index.index) = position;
            *self
                .components
                .chunks
                .get_unchecked_mut(generational_index.index) = chunk;
            *self
                .components
                .static_sprites
                .get_unchecked_mut(generational_index.index) = Some(tile.sprite);
            *self
                .components
                .palettes
                .get_unchecked_mut(generational_index.index) = tile.palette;
        }

        Some(generational_index)
    }

    pub(crate) fn register_grab(
        &mut self,
        position: components::Position,
        chunk: components::Chunk,
        bounding_box: components::BoundingBox,
        grabber: components::EntityReference,
    ) -> Option<GenerationalIndex> {
        let generational_index = self.generational_index_allocator.allocate()?;

        unsafe {
            *self.entities.get_unchecked_mut(generational_index.index) = Entity::position()
                | Entity::chunk()
                | Entity::bounding_box()
                | Entity::grab()
                | Entity::generation();

            *self
                .components
                .positions
                .get_unchecked_mut(generational_index.index) = position;
            *self
                .components
                .chunks
                .get_unchecked_mut(generational_index.index) = chunk;
            *self
                .components
                .bounding_boxes
                .get_unchecked_mut(generational_index.index) = bounding_box;
            *self
                .components
                .grabs
                .get_unchecked_mut(generational_index.index) = grabber;
            *self
                .components
                .generations
                .get_unchecked_mut(generational_index.index) = generational_index.generation;
        }

        Some(generational_index)
    }

    pub fn tick<T, RT>(
        &mut self,
        canvas: &mut Canvas<RT>,
        texture_creator: &'a TextureCreator<T>,
        event_pump: &mut EventPump,
        texture_cache: &mut LruCache<
            (ByAddress<&'a components::Sprite>, components::Palette),
            Texture<'a>,
        >,
    ) -> bool
    where
        RT: RenderTarget,
    {
        let mut events = Events::default();

        events |= system::event_handler(event_pump);

        if matches!(self.resources.game_state, components::GameState::Playing) {
            events |= system::reset_shield_use(self);

            events |= system::player_input(self, events.unwrap_input().unwrap_or(Input::default()));
            events |= system::follow_player(self);

            events |= system::toggle_walking_animation_state(self);

            for (entity, movement_delay, speed) in izip!(
                self.entities.iter(),
                self.components.movement_delays.iter_mut(),
                self.components.speeds.iter(),
            ) {
                if entity.has_movement_delay() && entity.has_speed() {
                    events |= system::decrement_movement_delay(movement_delay, *speed);
                }
            }
            events |= system::decrement_damage_invulnerability_timer(self);
            events |= system::decrement_use_cooldown(self);
            events |= system::decrement_retreating(self);

            events |= system::collisions(self);

            events |= system::cleanup_grabs(self);
            events |= system::clean_up_dead(self);
        }
        if matches!(self.resources.game_state, components::GameState::GameOver) {
            events |= system::remove_all_but_player(self);
            events |= system::restart(self, events.unwrap_input().unwrap_or(Input::default()));
        }

        events |= system::display_static_sprites(self, canvas, texture_creator, texture_cache);
        events |= system::display_sprites(self, canvas, texture_creator, texture_cache);
        events |= system::display_hud(self, canvas, texture_creator, texture_cache);

        !events.has_sys_exit()
    }
}
