use amethyst::core::nalgebra::Vector3;
use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage};
use ncollide2d::shape;

pub struct Ball {
    pub ball: shape::Ball<f32>,
}

pub struct Block {
    pub life: i32,
}

#[derive(Default)]
pub struct Bounced;

pub struct Cube(pub shape::Cuboid<f32>);

pub struct Paddle {
    pub speed: f32,
}

pub struct Direction(pub Vector3<f32>);

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Bounced {
    type Storage = NullStorage<Self>;
}

impl Component for Cube {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Direction {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}
