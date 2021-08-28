use crate::{
    components, constants, data,
    events::Events,
    system::{self, display_sort, find_pixel_difference},
    World,
};
use anyhow::anyhow;
use by_address::ByAddress;
use itertools::izip;
use lru::LruCache;
use sdl2::{
    pixels::PixelFormatEnum,
    rect::Rect,
    render::{BlendMode, Canvas, RenderTarget, Texture, TextureCreator},
    surface::Surface,
};

pub(crate) fn display_sprites<'a, T, RT, const ENTITY_COUNT: usize>(
    world: &mut World<'a, ENTITY_COUNT>,
    canvas: &mut Canvas<RT>,
    texture_creator: &'a TextureCreator<T>,
    texture_cache: &mut LruCache<
        (ByAddress<&'a components::Sprite>, components::Palette),
        Texture<'a>,
    >,
) -> Events
where
    RT: RenderTarget,
{
    let mut events = Events::default();

    for index in display_sort(world, |(index, entity)| {
        if entity.has_spritesheet_1x1() && entity.has_palette() {
            Some(index)
        } else {
            None
        }
    }) {
        let entity = unsafe { world.entities.get_unchecked(index) };
        let position = unsafe { world.components.positions.get_unchecked(index) };
        let chunk = unsafe { world.components.chunks.get_unchecked(index) };
        let spritesheet_1x1 = unsafe { world.components.spritesheets_1x1.get_unchecked(index) };
        let palette = unsafe { world.components.palettes.get_unchecked(index) };
        let walking_animation_state = if entity.has_walking_animation_state() {
            *unsafe {
                world
                    .components
                    .walking_animation_states
                    .get_unchecked(index)
            }
        } else {
            components::WalkingAnimationState::default()
        };

        let x = find_pixel_difference(
            position.x,
            chunk.x,
            world.resources.position.x,
            world.resources.chunk.x,
            constants::CHUNK_WIDTH,
        ) + constants::SCREEN_WIDTH as i16 / 2;
        let y = find_pixel_difference(
            position.y,
            chunk.y,
            world.resources.position.y,
            world.resources.chunk.y,
            constants::CHUNK_HEIGHT,
        ) + constants::SCREEN_HEIGHT as i16 / 2;

        // Just draw up for now.
        events |= system::display_sprite(
            unsafe {
                *spritesheet_1x1
                    .unwrap()
                    .down
                    .get_unchecked(0)
                    .get_unchecked(0)
                    .get_unchecked(walking_animation_state.to_index())
            },
            *palette,
            x as i32,
            y as i32,
            canvas,
            texture_creator,
            texture_cache,
        );
    }

    events
}
