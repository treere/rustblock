use amethyst::{
    assets::Handle,
    core::{nalgebra::Vector3, Transform},
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{Camera, DisplayConfig, Projection, VirtualKeyCode},
    ui::{Anchor, FontAsset, UiText, UiTransform},
};

use crate::component::{Ball, Block, Cube, Direction, Paddle};
use crate::config::{BallConfig, BlockConfig, PaddleConfig};
use crate::intro;
use crate::pause::Pause;
use crate::resources::{Lifes, MaterialVector, WindowSize};
use crate::util::*;

use ncollide2d::{math, shape};

pub struct Level;

#[derive(PartialEq)]
pub enum GameState {
    Running,
    Menu,
}
impl Default for GameState {
    fn default() -> Self {
        GameState::Menu
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Level {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        *world.write_resource() = GameState::Running;

        initialize_camera(world);
        initialize_pad(world);
        initialize_ball(world);
        initialize_block(world);
        initialize_lifes(world);
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world);

        if data.world.read_storage::<Block>().is_empty() {
            return Trans::Switch(Box::new(intro::Intro { ui: None }));
        }

        if !data.world.read_storage::<Ball>().is_empty() {
            return Trans::None;
        }

        let lifes = data.world.write_resource::<Lifes>().lifes - 1;

        if let Some(e) = data.world.read_resource::<Lifes>().e {
            if let Some(text) = data.world.write_storage::<UiText>().get_mut(e) {
                text.text = format!("{}", lifes).to_string();
            }
        }

        if lifes == 0 {
            Trans::Switch(Box::new(intro::Intro { ui: None }))
        } else {
            initialize_ball(data.world);
            data.world.write_resource::<Lifes>().lifes = lifes;
            Trans::None
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        data.world.delete_all();
        *data.world.write_resource() = GameState::Menu;
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        *data.world.write_resource() = GameState::Running;
    }

    fn handle_event(
        &mut self,
        data: StateData<GameData>,
        event: StateEvent,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::P) {
                *data.world.write_resource() = GameState::Menu;
                Trans::Push(Box::new(Pause { ui: None }))
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}

fn initialize_camera(world: &mut World) {
    let (width, height) = {
        let conf = world.read_resource::<DisplayConfig>();
        let (w, h) = conf.dimensions.unwrap();
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
    *world.write_resource() = WindowSize { width, height };
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
        let (w, _) = conf.dimensions.unwrap();
        trans.set_xyz((w as f32) * 0.5 - width * 0.5, height + offset, 0.);
    }

    let pad = Paddle { speed };

    let cube = Cube(shape::Cuboid::new(math::Vector::new(
        0.5 * width,
        0.5 * height,
    )));

    world
        .create_entity()
        .with(pad_mesh)
        .with(cube)
        .with(pad_material)
        .with(trans)
        .with(pad)
        .with(Direction(Vector3::new(0f32, 0f32, 0f32)))
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
        let (w, h) = conf.dimensions.unwrap();
        trans.set_xyz((w as f32) / 2. - radius, (h as f32) / 2. - radius, 0.);
    };

    let ball = Ball {
        ball: shape::Ball::new(radius),
    };

    world
        .create_entity()
        .with(pad_mesh)
        .with(pad_material)
        .with(trans)
        .with(ball)
        .with(Direction(Vector3::new(speed, speed, 0f32)))
        .build();
}

fn initialize_block(world: &mut World) {
    let (width, height) = {
        let config = world.read_resource::<BlockConfig>();
        (config.width, config.height)
    };
    let width_off = {
        let conf = world.read_resource::<DisplayConfig>();
        let (w, _) = conf.dimensions.unwrap();

        ((w as f32) - 10f32 * width) / 11f32
    };
    for rows in 0..10 {
        for cols in 0..3 {
            let pad_mesh = create_mesh(world, generate_rectangle_vertices(0.0, 0.0, width, height));

            let life = cols + 1;
            let material = {
                let m = world.read_resource::<MaterialVector>();
                m.lifes[life + 1].clone()
            };

            let mut trans = Transform::default();

            let x = width_off + (width + width_off) * (rows as f32);
            let y = 400f32 + (cols as f32) * (height + 10f32);
            trans.set_xyz(x, y, 0.);

            let block = Block { life: life as i32 };

            let cube = Cube(shape::Cuboid::new(math::Vector::new(
                0.5 * width,
                0.5 * height,
            )));

            world
                .create_entity()
                .with(pad_mesh)
                .with(cube)
                .with(material)
                .with(trans)
                .with(block)
                .build();
        }
    }
}

fn initialize_lifes(world: &mut World) {
    let lifes = match world.read_resource::<Lifes>().lifes {
        0 => 3,
        x => x,
    };

    let font = world.read_resource::<Handle<FontAsset>>().clone();

    let transform = UiTransform::new(
        "P1".to_string(),
        Anchor::BottomRight,
        -50.,
        50.,
        1.,
        50.,
        50.,
        0,
    );
    let text = UiText::new(
        font,
        format!("{}", lifes).to_string(),
        [1., 1., 1., 1.],
        50.,
    );
    let e = world.create_entity().with(transform).with(text).build();

    world.write_resource::<Lifes>().lifes = lifes;
    world.write_resource::<Lifes>().e.replace(e);
}
