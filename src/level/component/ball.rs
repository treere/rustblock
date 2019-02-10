use amethyst::core::nalgebra::Vector3;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Ball {
    pub radius: f32,
    pub vel: Vector3<f32>,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
