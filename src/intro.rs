use amethyst::{
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::VirtualKeyCode,
};

use crate::dispatcher::CustomGameData;
use crate::level::Level;

pub struct Intro;

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Intro {
    fn on_start(&mut self, _data: StateData<'_, CustomGameData<'_, '_>>) {}

    fn update(
        &mut self,
        data: StateData<CustomGameData>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, false);
        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::Return) {
                Trans::Push(Box::new(Level))
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}
