use amethyst::{
    assets::Handle,
    ecs::Entity,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::VirtualKeyCode,
    ui::{Anchor, FontAsset, UiText, UiTransform},
};

use crate::level::Level;
use crate::resources::Lifes;

pub struct Intro {
    pub ui: Option<Entity>,
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Intro {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.write_resource::<Lifes>().lifes = 3;

        let font = world.read_resource::<Handle<FontAsset>>().clone();

        let transform =
            UiTransform::new("P1".to_string(), Anchor::Middle, 0., 0., 1., 400., 100., 0);

        let ui = UiText::new(font, "RustBlocks".to_string(), [1., 1., 1., 1.], 50.);

        self.ui = Some(world.create_entity().with(transform).with(ui).build());
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(ent) = self.ui.take() {
            data.world.delete_entity(ent).expect("Cannot delete ui");
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world);
        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData>,
        event: StateEvent,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::Return) {
                Trans::Switch(Box::new(Level))
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}
