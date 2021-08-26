use crate::components;

pub const SCREEN_WIDTH: u32 = 256;
pub const SCREEN_HEIGHT: u32 = 240;

pub(crate) const HUD_WIDTH: u32 = 256;
pub(crate) const HUD_HEIGHT: u32 = 16;

pub(crate) const CHUNK_WIDTH: u16 = 512;
pub(crate) const CHUNK_HEIGHT: u16 = 512;
pub(crate) const CHUNK_TILE_WIDTH: usize = 32;
pub(crate) const CHUNK_TILE_HEIGHT: usize = 32;

pub(crate) const WORLD_WIDTH: u8 = 255;
pub(crate) const WORLD_HEIGHT: u8 = 255;

pub(crate) const SPRITE_WIDTH: u32 = 16;
pub(crate) const SPRITE_HEIGHT: u32 = 16;

pub(crate) const SCREEN_CENTER_HALF_WIDTH: u16 = 48;
pub(crate) const SCREEN_CENTER_HALF_HEIGHT: u16 = 32;

pub(crate) const STARTING_CHUNK: components::Chunk = components::Chunk { x: 10, y: 10 };
pub(crate) const STARTING_POSITION: components::Position = components::Position { x: 100, y: 90 };
