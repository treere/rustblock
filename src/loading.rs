use crate::dispatcher::CustomGameData;
use crate::intro::Intro;
use crate::resources::Lifes;
use crate::resources::MaterialVector;
use crate::util::*;
use amethyst::{assets::Loader, input::is_close_requested, prelude::*, ui::TtfFormat};

pub struct Loading;

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Loading {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        let world = data.world;
        initialize_colors(world);

        world.add_resource(Lifes::default());

        let font = world.read_resource::<Loader>().load(
            "font/square.ttf",
            TtfFormat,
            (),
            (),
            &world.read_resource(),
        );
        world.add_resource(font);
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true);
        Trans::Switch(Box::new(Intro { ui: None }))
    }

    fn handle_event(
        &mut self,
        _data: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}

fn initialize_colors(world: &mut World) {
    let m = MaterialVector {
        pad: create_colour_material(world, [0., 0., 1., 1.]),
        ball: create_colour_material(world, [0.5, 0.5, 0.5, 0.5]),
        lifes: vec![
            create_colour_material(world, [1., 0., 1., 1.]),
            create_colour_material(world, [1., 1., 1., 1.]),
            create_colour_material(world, [0., 1., 1., 1.]),
            create_colour_material(world, [1., 1., 0., 1.]),
            create_colour_material(world, [1., 0., 0., 1.]),
        ],
    };
    world.add_resource(m);
}
