//#![windows_subsystem = "windows"]

use lru::LruCache;
use sdl2::gfx::framerate::FPSManager;
use shield::{constants, data, World};

pub fn main() -> anyhow::Result<()> {
    let sdl_context = sdl2::init().map_err(|e| anyhow::format_err!(e))?;
    let video_subsystem = sdl_context.video().map_err(|e| anyhow::format_err!(e))?;

    // TODO: Increase resolution, set full screen mode.
    let window = video_subsystem
        .window("shield", constants::SCREEN_WIDTH, constants::SCREEN_HEIGHT)
        .resizable()
        .build()?;

    let mut canvas = window.into_canvas().build()?;
    canvas.set_logical_size(constants::SCREEN_WIDTH, constants::SCREEN_HEIGHT)?;

    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context
        .event_pump()
        .map_err(|e| anyhow::format_err!(e))?;

    let mut fps_manager = FPSManager::new();
    fps_manager
        .set_framerate(60)
        .map_err(|e| anyhow::format_err!(e))?;

    let mut texture_cache = LruCache::new(256);

    let mut world = World::<8192>::default();
    world.new_game()?;

    loop {
        canvas.set_draw_color(data::colors::VOID.to_tuple());
        canvas.clear();

        if !world.tick(&mut canvas, &texture_creator, &mut event_pump, &mut texture_cache) {
            break;
        }

        canvas.present();
        fps_manager.delay();
    }

    Ok(())
}
