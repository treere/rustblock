use amethyst::ecs::Entity;
use amethyst::renderer::Material;

pub struct MaterialVector {
    pub pad: Material,
    pub ball: Material,
    pub lifes: Vec<Material>,
}

#[derive(Default)]
pub struct Lifes {
    pub lifes: u32,
    pub e: Option<Entity>,
}

#[derive(Default)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}
