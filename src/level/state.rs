use amethyst::{
    core::{nalgebra::Vector3, Transform},
    prelude::*,
    renderer::{Camera, Projection},
};

use super::{BALL_RADIUS, BALL_SPEED, BLOCK_HEIGHT, BLOCK_WIDTH, PADDLE_HEIGHT, PADDLE_OFFSET, PADDLE_SPEED, PADDLE_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};
use super::component::{Ball, Block, Paddle};
use super::resources::MaterialVector;
use super::util::*;

pub struct Level;

impl SimpleState for Level {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Paddle>();
        world.register::<Ball>();
        world.register::<Block>();

        let m = MaterialVector {
            pad: Some(create_colour_material(world, [0., 0., 1., 1.])),
            ball: Some(create_colour_material(world, [0.5, 0.5, 0.5, 0.5])),
            lifes: vec![
                create_colour_material(world, [1., 0., 1., 1.]),
                create_colour_material(world, [1., 1., 1., 1.]),
                create_colour_material(world, [0., 1., 1., 1.]),
                create_colour_material(world, [1., 1., 0., 1.]),
                create_colour_material(world, [1., 0., 0., 1.])
            ]
        };

        world.add_resource(m);

        initialize_camera(world);
        initialize_pad(world);
        initialize_ball(world);
        initialize_block(world);
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

    let pad_material = {
        let m = world.read_resource::<MaterialVector>();
        m.pad.clone().unwrap()
    };

    let mut trans = Transform::default();
    trans.set_xyz(SCREEN_WIDTH / 2. - PADDLE_WIDTH / 2., PADDLE_HEIGHT + PADDLE_OFFSET, 0.);

    let pad = Paddle { width: PADDLE_WIDTH, height: PADDLE_HEIGHT, speed: PADDLE_SPEED };

    world
        .create_entity()
        .with(pad_mesh)
        .with(pad_material)
        .with(trans)
        .with(pad)
        .build();
}

fn initialize_ball(world: &mut World) {
    let pad_mesh = create_mesh(world, generate_circle_vertices(BALL_RADIUS, 16));

    let pad_material = {
        let m = world.read_resource::<MaterialVector>();
        m.ball.clone().unwrap()
    };

    let mut trans = Transform::default();
    trans.set_xyz(SCREEN_WIDTH / 2. - BALL_RADIUS, SCREEN_HEIGHT / 2. - BALL_RADIUS, 0.);

    let ball = Ball { radius: BALL_RADIUS, vel: Vector3::new(BALL_SPEED, BALL_SPEED, 0f32) };

    world
        .create_entity()
        .with(pad_mesh)
        .with(pad_material)
        .with(trans)
        .with(ball)
        .build();
}

fn initialize_block(world: &mut World) {
    let width_off = (SCREEN_WIDTH - 10f32 * BLOCK_WIDTH) / 11f32;
    for rows in 0..10 {
        for cols in 0..3 {
            let pad_mesh = create_mesh(world, generate_rectangle_vertices(0.0, 0.0, BLOCK_WIDTH, BLOCK_HEIGHT));

            let life = cols + 1;
            let block_material = {
                let m = world.read_resource::<MaterialVector>();
                m.lifes[life + 1].clone()
            };

            let mut trans = Transform::default();

            let x = width_off + (BLOCK_WIDTH + width_off) * (rows as f32);
            let y = 400f32 + (cols as f32) * (BLOCK_HEIGHT + 10f32);
            trans.set_xyz(x, y, 0.);

            let block = Block { width: BLOCK_WIDTH, height: BLOCK_HEIGHT, life: life as i32 };

            world
                .create_entity()
                .with(pad_mesh)
                .with(block_material)
                .with(trans)
                .with(block)
                .build();
        }
    }
}
