use crate::events::{Events, Input};
use sdl2::{event::Event, keyboard::Scancode, EventPump};

pub(crate) fn event_handler(event_pump: &mut EventPump) -> Events {
    let mut output_events = Events::default();

    for event in event_pump.poll_iter() {
        output_events |= match event {
            Event::Quit { .. } => Events::sys_exit(),
            _ => Events::default(),
        }
    }

    let keyboard_state = event_pump.keyboard_state();
    if keyboard_state.is_scancode_pressed(Scancode::Up) {
        output_events |= Events::input(Input::UP);
    }
    if keyboard_state.is_scancode_pressed(Scancode::Right) {
        output_events |= Events::input(Input::RIGHT);
    }
    if keyboard_state.is_scancode_pressed(Scancode::Down) {
        output_events |= Events::input(Input::DOWN);
    }
    if keyboard_state.is_scancode_pressed(Scancode::Left) {
        output_events |= Events::input(Input::LEFT);
    }
    if keyboard_state.is_scancode_pressed(Scancode::Escape) {
        output_events |= Events::input(Input::START);
    }
    if keyboard_state.is_scancode_pressed(Scancode::LShift) {
        output_events |= Events::input(Input::R);
    }
    if keyboard_state.is_scancode_pressed(Scancode::X) {
        output_events |= Events::input(Input::A);
    }
    if keyboard_state.is_scancode_pressed(Scancode::Z) {
        output_events |= Events::input(Input::B);
    }

    output_events
}
