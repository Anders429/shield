use crate::{components, constants, events::Events};
use by_address::ByAddress;
use lru::LruCache;
use sdl2::{
    pixels::PixelFormatEnum,
    rect::Rect,
    render::{BlendMode, Canvas, RenderTarget, TextureCreator, Texture},
    surface::Surface,
};

pub(crate) fn display_sprite<'a, T, RT>(
    sprite: &'a components::Sprite,
    palette: components::Palette,
    x: i32,
    y: i32,
    canvas: &mut Canvas<RT>,
    texture_creator: &'a TextureCreator<T>,
    texture_cache: &mut LruCache<(ByAddress<&'a components::Sprite>, components::Palette), Texture<'a>>,
) -> Events
where
    RT: RenderTarget,
{
    let texture = match texture_cache.get(&(ByAddress(sprite), palette)) {
        Some(texture) => texture,
        None => {
            let mut texture = match texture_creator.create_texture_streaming(
                PixelFormatEnum::BGRA8888,
                constants::SPRITE_WIDTH,
                constants::SPRITE_HEIGHT,
            ) {
                Ok(texture) => texture,
                Err(_) => return Events::default(),
            };
            texture.set_blend_mode(BlendMode::Blend);
            let mut raw_pixels = sprite.to_argb32_pixels(&palette);
            match texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for (i, v) in raw_pixels.iter().enumerate() {
                    buffer[i] = *v;
                }
            }) {
                Ok(_) => {}
                Err(_) => return Events::default(),
            };

            texture_cache.put((ByAddress(sprite), palette), texture);

            &texture_cache.get(&(ByAddress(sprite), palette)).unwrap()
        }
    };
    
    match canvas.copy(
        &texture,
        None,
        Some(Rect::new(
            x,
            y,
            constants::SPRITE_WIDTH,
            constants::SPRITE_HEIGHT,
        )),
    ) {
        Ok(_) => {}
        Err(_) => return Events::default(),
    };

    Events::default()
}
