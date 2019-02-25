use amethyst::{
    assets::Handle,
    ecs::Entity,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::VirtualKeyCode,
    ui::{Anchor, FontAsset, UiText, UiTransform},
};

use crate::dispatcher::CustomGameData;

pub struct Pause {
    pub ui: Option<Entity>,
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Pause {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        let world = data.world;

        let font = world.read_resource::<Handle<FontAsset>>().clone();

        let transform =
            UiTransform::new("P1".to_string(), Anchor::Middle, 0., 0., 1., 400., 100., 0);

        let ui = UiText::new(font, "PAUSE".to_string(), [1., 1., 1., 1.], 50.);

        self.ui = Some(world.create_entity().with(transform).with(ui).build());
    }

    fn on_stop(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        if let Some(ent) = self.ui.take() {
            data.world.delete_entity(ent).expect("Cannot delete ui");
        }
    }

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
            } else if is_key_down(&event, VirtualKeyCode::P) {
                Trans::Pop
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}
