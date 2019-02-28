use amethyst::ecs::prelude::{Component, DenseVecStorage};
use ncollide2d::shape;

pub struct Paddle {
    pub paddle: shape::Cuboid<f32>,
    pub speed: f32,
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}
