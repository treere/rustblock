use amethyst::ecs::prelude::{Component, NullStorage};

#[derive(Default)]
pub struct LevelComponent;

impl Component for LevelComponent {
    type Storage = NullStorage<Self>;
}
