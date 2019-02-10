use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Block {
    pub width: f32,
    pub height: f32,
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}
