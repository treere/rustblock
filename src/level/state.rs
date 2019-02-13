use amethyst::{
    core::{nalgebra::Vector3, Transform},
    prelude::*,
    renderer::{Camera, DisplayConfig, Projection},
};

use super::component::{Ball, Block, Paddle};
use super::config::{BallConfig, BlockConfig, PaddleConfig};
use super::resources::MaterialVector;
use super::util::*;

pub struct Level;

impl SimpleState for Level {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialize_colors(world);
        initialize_camera(world);
        initialize_pad(world);
        initialize_ball(world);
        initialize_block(world);
    }
}

fn initialize_camera(world: &mut World) {
    let (width, height) = {
        let conf = world.read_resource::<DisplayConfig>();
        let (w, h) = conf.dimensions.clone().unwrap();
        (w as f32, h as f32)
    };

    let mut transform = Transform::default();

    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0, width, 0.0, height,
        )))
        .with(transform)
        .build();
}

fn initialize_pad(world: &mut World) {
    let (width, height, offset, speed) = {
        let config = world.read_resource::<PaddleConfig>();
        (config.width, config.height, config.offset, config.speed)
    };

    let pad_mesh = create_mesh(world, generate_rectangle_vertices(0.0, 0.0, width, height));

    let pad_material = {
        let m = world.read_resource::<MaterialVector>();
        m.pad.clone()
    };

    let mut trans = Transform::default();
    {
        let conf = world.read_resource::<DisplayConfig>();
        let (w, _) = conf.dimensions.clone().unwrap();
        trans.set_xyz((w as f32) / 2. - width / 2., height + offset, 0.);
    }

    let pad = Paddle {
        width: width,
        height: height,
        speed: speed,
    };

    world
        .create_entity()
        .with(pad_mesh)
        .with(pad_material)
        .with(trans)
        .with(pad)
        .build();
}

fn initialize_ball(world: &mut World) {
    let (speed, radius) = {
        let config = world.read_resource::<BallConfig>();
        (config.speed, config.radius)
    };

    let pad_mesh = create_mesh(world, generate_circle_vertices(radius, 16));

    let pad_material = {
        let m = world.read_resource::<MaterialVector>();
        m.ball.clone()
    };

    let mut trans = Transform::default();
    {
        let conf = world.read_resource::<DisplayConfig>();
        let (w, h) = conf.dimensions.clone().unwrap();
        trans.set_xyz((w as f32) / 2. - radius, (h as f32) / 2. - radius, 0.);
    };

    let ball = Ball {
        radius: radius,
        vel: Vector3::new(speed, speed, 0f32),
    };

    world
        .create_entity()
        .with(pad_mesh)
        .with(pad_material)
        .with(trans)
        .with(ball)
        .build();
}

fn initialize_block(world: &mut World) {
    let (width, height) = {
        let config = world.read_resource::<BlockConfig>();
        (config.width, config.height)
    };
    let width_off = {
        let conf = world.read_resource::<DisplayConfig>();
        let (w, _) = conf.dimensions.clone().unwrap();

        ((w as f32) - 10f32 * width) / 11f32
    };
    for rows in 0..10 {
        for cols in 0..3 {
            let pad_mesh = create_mesh(world, generate_rectangle_vertices(0.0, 0.0, width, height));

            let life = cols + 1;
            let block_material = {
                let m = world.read_resource::<MaterialVector>();
                m.lifes[life + 1].clone()
            };

            let mut trans = Transform::default();

            let x = width_off + (width + width_off) * (rows as f32);
            let y = 400f32 + (cols as f32) * (height + 10f32);
            trans.set_xyz(x, y, 0.);

            let block = Block {
                width: width,
                height: height,
                life: life as i32,
            };

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
