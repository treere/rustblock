use amethyst::renderer::Material;

pub struct MaterialVector {
    pub pad: Option<Material>,
    pub ball: Option<Material>,
    pub lifes: Vec<Material>,
}

impl Default for MaterialVector {
    fn default() -> Self {
        MaterialVector {
            pad: None,
            ball: None,
            lifes: vec![]
        }
    }
}