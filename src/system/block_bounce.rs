use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, Join, ReadStorage, System, WriteStorage},
};

use super::bounce_util::{bounce, Collision};
use crate::component::{Ball, Block, Bounced};
pub struct BounceBlock;

impl<'s> System<'s> for BounceBlock {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Block>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Bounced>,
    );

    fn run(&mut self, (entities, mut balls, mut blocks, transforms, mut bounc): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            for (block, transformb, entity) in (&mut blocks, &transforms, &entities).join() {
                if let Some(vertical) = bounce(
                    transformb,
                    transform,
                    ball.radius,
                    block.width,
                    block.height,
                ) {
                    match vertical {
                        Collision::Vertical => ball.vel[1] = -ball.vel[1],
                        Collision::Horizontal => ball.vel[0] = -ball.vel[0],
                    };
                    bounc.insert(entity, Bounced).unwrap();
                }
            }
        }
    }
}
