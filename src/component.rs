use amethyst::core::nalgebra::Vector3;
use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage};
use ncollide2d::shape;

pub struct Ball {
    pub ball: shape::Ball<f32>,
    pub vel: Vector3<f32>,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub struct Block {
    pub life: i32,
}

pub struct Cube(pub shape::Cuboid<f32>);

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Cube {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Bounced;

impl Component for Bounced {
    type Storage = NullStorage<Self>;
}

pub struct Paddle {
    pub vel: Vector3<f32>,
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}
