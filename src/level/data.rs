use amethyst::{
    core::Transform,
    prelude::*,
    renderer::{Camera, Projection},
};

use super::util::*;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;
const BALL_RADIUS: f32 = 5.;
const PADDLE_WIDTH: f32 = 100.;
const PADDLE_HEIGHT: f32 = 10.;
const PADDLE_OFFSET: f32 = 10.;

pub struct Level;

impl SimpleState for Level {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialize_camera(world);
        initialize_pad(world);
        initialize_ball(world);
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();

    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(0.0, SCREEN_WIDTH, 0.0, SCREEN_HEIGHT)))
        .with(transform)
        .build();
}

fn initialize_pad(world: &mut World) {
    let pad_mesh = create_mesh(world, generate_rectangle_vertices(0.0, 0.0, PADDLE_WIDTH, PADDLE_HEIGHT));

    let pad_material = create_colour_material(world, [0., 0., 1., 1.]);

    let mut trans = Transform::default();
    trans.set_xyz(SCREEN_WIDTH / 2. - PADDLE_WIDTH / 2., PADDLE_HEIGHT + PADDLE_OFFSET, 0.);

    world
        .create_entity()
        .with(pad_mesh)
        .with(pad_material)
        .with(trans)
        .build();
}

fn initialize_ball(world: &mut World) {
    let pad_mesh = create_mesh(world, generate_circle_vertices(BALL_RADIUS, 16));

    let pad_material = create_colour_material(world, [1., 0., 1., 1.]);

    let mut trans = Transform::default();
    trans.set_xyz(SCREEN_WIDTH / 2. - BALL_RADIUS, SCREEN_HEIGHT / 2. - BALL_RADIUS, 0.);

    world
        .create_entity()
        .with(pad_mesh)
        .with(pad_material)
        .with(trans)
        .build();
}
