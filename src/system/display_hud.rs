use crate::{components, constants, data, events::Events, system, World};
use by_address::ByAddress;
use itertools::izip;
use lru::LruCache;
use sdl2::{
    rect::Rect,
    render::{Canvas, RenderTarget, Texture, TextureCreator},
};

pub(crate) fn display_hud<'a, T, RT, const ENTITY_COUNT: usize>(
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
    canvas.set_draw_color(data::colors::VOID.to_tuple());
    canvas.fill_rect(Rect::new(0, 0, constants::HUD_WIDTH, constants::HUD_HEIGHT));

    let full_palette = components::Palette {
        color_a: data::colors::MEAT,
        color_b: data::colors::RED,
        color_c: data::colors::NIGHTBLUE,
    };
    let empty_palette = components::Palette {
        color_a: data::colors::CLOUDBLUE,
        color_b: data::colors::GRAY,
        color_c: data::colors::NIGHTBLUE,
    };

    for (entity, health_points) in
        izip!(world.entities.iter(), world.components.health_points.iter())
    {
        if entity.has_health_points() && entity.is_player() {
            for i in 0..health_points.max {
                if i < health_points.current {
                    system::display_sprite(
                        &data::sprites::HEART,
                        full_palette,
                        16 * i as i32,
                        0,
                        canvas,
                        texture_creator,
                        texture_cache,
                    );
                } else {
                    system::display_sprite(
                        &data::sprites::HEART,
                        empty_palette,
                        16 * i as i32,
                        0,
                        canvas,
                        texture_creator,
                        texture_cache,
                    );
                }
            }
            break;
        }
    }

    Events::default()
}
