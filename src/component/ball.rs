use amethyst::core::nalgebra::Vector3;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use ncollide2d::shape;

pub struct Ball {
    pub ball: shape::Ball<f32>,
    pub vel: Vector3<f32>,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
