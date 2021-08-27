use crate::{
    components, constants, data,
    events::Events,
    system::{self, find_pixel_difference},
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

pub(crate) fn display_static_sprites<'a, T, RT, const ENTITY_COUNT: usize>(
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

    for (entity, position, chunk, static_sprite, palette) in izip!(
        world.entities.iter(),
        world.components.positions.iter(),
        world.components.chunks.iter(),
        world.components.static_sprites.iter(),
        world.components.palettes.iter(),
    ) {
        if entity.has_position()
            && entity.has_chunk()
            && entity.has_static_sprite()
            && entity.has_palette()
        {
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

            events |= system::display_sprite(
                static_sprite.unwrap(),
                *palette,
                x as i32,
                y as i32,
                canvas,
                texture_creator,
                texture_cache,
            );
        }
    }

    events
}
