use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage};

pub struct Block {
    pub width: f32,
    pub height: f32,
    pub life: i32,
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Bounced;

impl Component for Bounced {
    type Storage = NullStorage<Self>;
}
