use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Ball {
    pub radius: f32
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub struct Paddle {
    pub width: f32,
    pub height: f32
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}