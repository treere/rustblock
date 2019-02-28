use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage};
use ncollide2d::shape;

pub struct Block {
    pub block: shape::Cuboid<f32>,
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
