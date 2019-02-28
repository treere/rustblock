use amethyst::{
    ecs::prelude::{Entities, Join, ReadExpect, System, WriteStorage},
    renderer::Material,
};

use crate::component::{Block, Bounced};
use crate::resources::MaterialVector;

pub struct BouncedBlock;

impl<'s> System<'s> for BouncedBlock {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Block>,
        WriteStorage<'s, Bounced>,
        WriteStorage<'s, Material>,
        ReadExpect<'s, MaterialVector>,
    );

    fn run(&mut self, (entities, mut blocks, mut bounced, mut mat, matvec): Self::SystemData) {
        for (block, _, entity) in (&mut blocks, &bounced, &entities).join() {
            if block.life > 0 {
                block.life -= 1;
            }
            if block.life == 0 {
                entities.delete(entity).unwrap();
            } else {
                mat.remove(entity).unwrap();
                let color = matvec.lifes[block.life as usize + 1].clone();
                mat.insert(entity, color).unwrap();
            }
        }
        bounced.clear();
    }
}
