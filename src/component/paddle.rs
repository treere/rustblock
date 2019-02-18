use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Paddle {
    pub width: f32,
    pub height: f32,
    pub speed: f32,
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}
