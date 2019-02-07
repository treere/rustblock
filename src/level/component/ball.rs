use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Ball {
    pub radius: f32,
    pub vel_x: f32,
    pub vel_y: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
